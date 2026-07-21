# SGEIR

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a general system of linear equations. Iterative refinement is used to obtain an error estimate.

## Description

Subroutine SGEIR solves a general NxN system of single precision linear equations using LINPACK subroutines SGEFA and SGESL. One pass of iterative refinement is used only to obtain an estimate of the accuracy. That is, if A is an NxN real

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `conflicting`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/sgeir.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sgeir.f` (`live-main-source`)
  - `main-src-historical/src/sgeir.f.0` (`historical-variant`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sgeir.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sgeir.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Multiple or variant providers remain separate pending manual reconciliation.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [SGEIR](https://www.netlib.org/slatec/src/sgeir.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDA, *) | B. angular matrices U and L using partial pivoting.  These factors and the pivoting information are used to calculate the solution, X.  Then the residual vector is found and used to calculate an estimate of the relative error, IND. B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to solve only (ITASK .GT. 1) will be faster for the succeeding solutions.  In this case, the contents of A, REAL(LDA,N) the doubly subscripted array with dimension (LDA,N) which contains the coefficient matrix.  A is not altered by the routine. must be greater than or equal to 1. B. on return, V contains the solution vector, X . singly subscripted array of dimension at least N*(N+1). singly subscripted array of dimension at least N. solution has not been computed. may be poorly scaled. Note-  The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 .  LEVEL=0 for warning error messages from XERMSG.  Unless the user provides otherwise, an error message will be printed followed by an abort. |
| 2 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must not have been altered by the INTEGER |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | vectors, then SGEIR solves the equation must not have been altered by the 1) INTEGER the order of the matrix A.  The first N elements of the array A are the elements of the first column of must be greater than or equal to 1. |
| 4 | `V` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(N) on entry, the singly subscripted array(vector) of di- mension N which contains the right hand side B of a |
| 5 | `ITASK` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1).  IND will not be changed by SGEIR in this case. Argument Description *** INTEGER 1, the matrix A is factored and then the linear equation is solved. If ITASK .GT. 1, the equation is solved using the existing factored matrix A (stored in WORK). 3 is printed. |
| 6 | `IND` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | estimates the accuracy of the solution only when the input matrix and the right hand side are represented exactly in the computer and does not take into account any errors in the input data. 1) 2) 3 is printed. INTEGER GT. 0  IND is a rough estimate of the number of digits 75 means that the solution vector X is zero. LT. 0  see error message corresponding to IND below. 1  terminal   N is greater than LDA. 2  terminal   N is less than one. 3  terminal   ITASK is less than one. 4  terminal   The matrix A is computationally singular. 10 warning    The solution has no apparent significance. The solution may be inaccurate or the matrix |
| 7 | `WORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 2; dimensions (N, *) | must not have been altered by the REAL(N*(N+1)) |
| 8 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | must not have been altered by the INTEGER(N) |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`WORK`: must not have been altered by the REAL(N*(N+1))

`IWORK`: must not have been altered by the INTEGER(N)

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::sgeir`. Native symbol: `sgeir_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::sgeir`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
