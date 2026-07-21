# SPOFS

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a positive definite symmetric system of linear equations.

## Description

Subroutine SPOFS solves a real positive definite symmetric NxN system of single precision linear equations using LINPACK subroutines SPOCO and SPOSL. That is, if A is an NxN real positive definite symmetric matrix and if X and B

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
- GAMS classifications: `D2B1B`
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

- Canonical provider: `main-src/src/spofs.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/spofs.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/spofs.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/spofs.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [SPOFS](https://www.netlib.org/slatec/src/spofs.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDA, *) | B. angular matrices R and R-TRANSPOSE.  These factors are used to find the solution vector X.  An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to solve only (ITASK .GT. 1) will be faster for the succeeding solutions.  In this case, the contents of A, REAL(LDA,N) on entry, the doubly subscripted array with dimension TRANSPOSE) * R . must be greater B. on return, V contains the solution vector, X . singly subscripted array of dimension at least N. |
| 2 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must not have been altered by the user following contains the coefficient matrix.  Only the upper triangle, including the diagonal, of the coefficient matrix need be entered and will subse- quently be referenced and changed by the routine. on return, contains in its upper triangle an upper INTEGER |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | vectors, then SPOFS solves the equation must not have been altered by the user following contains the coefficient matrix.  Only the upper triangle, including the diagonal, of the coefficient matrix need be entered and will subse- quently be referenced and changed by the routine. on return, contains in its upper triangle an upper 1) INTEGER must be greater |
| 4 | `V` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(N) on entry, the singly subscripted array(vector) of di- mension N which contains the right hand side B of a |
| 5 | `ITASK` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1).  IND will not be changed by SPOFS in this case. Argument Description *** INTEGER 1, the matrix A is factored and then the linear equation is solved. If ITASK .GT. 1, the equation is solved using the existing factored matrix A. 3 is printed. |
| 6 | `IND` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1) 2) 3 is printed. INTEGER GT. 0  IND is a rough estimate of the number of digits of accuracy in the solution, X. LT. 0  see error message corresponding to IND below. 1  terminal   N is greater than LDA. 2  terminal   N is less than 1. 3  terminal   ITASK is less than 1. 4  Terminal   The matrix A is computationally singular or is not positive definite.  A solution has not been computed. 10 warning    The solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. Note-  The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 .  LEVEL=0 for warning error messages from XERMSG.  Unless the user provides otherwise, an error message will be printed followed by an abort. |
| 7 | `WORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(N) |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`WORK`: REAL(N)

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::spofs`. Native symbol: `spofs_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::spofs`
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
