# SSJAC

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Jacobi's Method Iterative Sparse Ax = b Solver. Routine to solve a general linear system Ax = b using Jacobi iteration.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, LENW, IWORK(LENIW), LENIW REAL B(N), X(N), A(NELT), TOL, ERR, RWORK(LENW) CALL SSJAC(N, B, X, NELT, IA, JA, A, ISYM, ITOL, TOL, $ ITMAX, ITER, ERR, IERR, IUNIT, RWORK, LENW, IWORK, LENIW ) Jacobi's method solves the linear system Ax=b with the basic iterative method (where A = L + D + U): n+1 -1 n n

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
- GAMS classifications: `D2A4`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/ssjac.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssjac.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/ssjac.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [SSJAC](https://www.netlib.org/slatec/lin/ssjac.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Order of the Matrix. -1 n = X + D (B - AX ) The Sparse Linear Algebra Package (SLAP) utilizes two matrix data structures: 1) the SLAP Triad format or 2) the SLAP Column format. The user can hand this routine either of the of these data structures and SLAP will figure out which one is being used and act accordingly. =================== S L A P Triad format =================== This routine requires that the matrix A be stored in the SLAP Triad format. In this format only the non-zeros are stored. |
| 2 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | B(N). Right-hand side vector. |
| 3 | `X` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution. D (B - LX - UX ). |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Number of Non-Zeros stored in A. |
| 5 | `IA` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | IA(NELT). |
| 6 | `JA` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | JA(NELT). NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '\|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 \|11 12 0 0 15\| A: 11 21 51 \| 22 12 \| 33 53 \| 44 \| 55 15 35 \|21 22 0 0 0\| IA: 1 2 5 \| 2 1 \| 3 5 \| 4 \| 5 1 3 \| 0 0 33 0 35\| JA: 1 4 6 8 9 12 \| 0 0 0 44 0\| \|51 0 53 0 55\| Side Effects: The SLAP Triad format (IA, JA, A) is modified internally to be the SLAP Column format. See above. Cautions: This routine will attempt to write to the Fortran logical output unit IUNIT, if IUNIT. |
| 7 | `A` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NELT) | A(NELT). These arrays should hold the matrix A in either the SLAP Triad format or the SLAP Column format. See "Description", below. If the SLAP Triad format is chosen it is changed internally to the SLAP Column format. |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. |
| 9 | `ITOL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Flag to indicate type of convergence criterion. If ITOL=1, iteration stops when the 2-norm of the residual divided by the 2-norm of the right-hand side is less than TOL. If ITOL=2, iteration stops when the 2-norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand side is less than TOL, where M-inv is the inverse of the diagonal of A. 11 is often useful for checking and comparing different routines. For this case, the user must supply the "exact" solution or a very accurate approximation (one with an error much less than TOL) through a common block, COMMON /SSLBLK/ SOLN( ) If ITOL=11, iteration stops when the 2-norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL. |
| 10 | `TOL` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Real. Convergence criterion, as described above. (Reset if IERR=4. ). |
| 11 | `ITMAX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Maximum number of iterations. |
| 12 | `ITER` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Number of iterations required to reach convergence, or ITMAX+1 if convergence criterion could not be achieved in ITMAX iterations. |
| 13 | `ERR` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | Real. Error estimate of error in final approximate solution, as defined by ITOL. |
| 14 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Return error flag. 0 => All went well. 1 => Insufficient space allocated for WORK or IWORK. 2 => Method failed to converge in ITMAX steps. 3 => Error in user input. |
| 15 | `IUNIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence. If unit number is 0, no writing will occur. |
| 16 | `RWORK` | `workspace-output` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (LENW) | RWORK(LENW). array used for workspace. |
| 17 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Length of the real workspace, RWORK. LENW >= 4*N. |
| 18 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (LENIW) | IWORK(LENIW). Used to hold pointers into the real workspace, RWORK. Upon return the following locations of IWORK hold information which may be of use to the user: Amount of Integer workspace actually used. IWORK(10) Amount of Real workspace actually used. |
| 19 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Integer. Length of the integer workspace, IWORK. LENIW >= 10. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and workspace requirements

`RWORK`: RWORK(LENW). array used for workspace.

`IWORK`: IWORK(LENIW). Used to hold pointers into the real workspace, RWORK. Upon return the following locations of IWORK hold information which may be of use to the user: Amount of Integer workspace actually used. IWORK(10) Amount of Real workspace actually used.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::sparse::ssjac`. Native symbol: `ssjac_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::sparse::ssjac`
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
