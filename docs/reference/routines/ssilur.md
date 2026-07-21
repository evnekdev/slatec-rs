# SSILUR

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Incomplete LU Iterative Refinement Sparse Ax = b Solver. Routine to solve a general linear system Ax = b using the incomplete LU decomposition with iterative refinement.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, LENW, IWORK(NL+NU+4*N+2), LENIW REAL B(N), X(N), A(NELT), TOL, ERR, RWORK(NL+NU+4*N) CALL SSILUR(N, B, X, NELT, IA, JA, A, ISYM, ITOL, TOL, $ ITMAX, ITER, ERR, IERR, IUNIT, RWORK, LENW, IWORK, LENIW) The Sparse Linear Algebra Package (SLAP) utilizes two matrix data structures: 1) the SLAP Triad format or 2) the SLAP Column format. The user can hand this routine either of the of these data structures and SLAP will figure out which on is being used and act accordingly. =================== S L A P Triad format =================== This routine requires that the matrix A be stored in the SLAP Triad format. In this format only the non-zeros are stored. They may appear in *ANY* order. The user supplies

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

- Canonical provider: `lin/ssilur.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssilur.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/ssilur.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [SSILUR](https://www.netlib.org/slatec/lin/ssilur.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer Order of the Matrix. NELT+1, where  N  is the number of columns in  the |
| 2 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | IN       Real B(N). Right-hand side vector. |
| 3 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | INOUT    Real X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution. |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Number of Non-Zeros stored in A. is  the number  of is  the number  of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)).  For non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)).  For each non-zero the user puts the row and column index of that each non-zero the user puts the row and column index of that matrix element  in the IA and  JA arrays.  The  value of the matrix element  in the IA and  JA arrays.  The  value of the non-zero   matrix  element is  placed  in  the corresponding non-zero   matrix  element is  placed  in  the corresponding location of the A array.   This is  an  extremely  easy data location of the A array.   This is  an  extremely  easy data structure to generate.  On  the  other hand it   is  not too structure to generate.  On  the  other hand it   is  not too efficient on vector computers for  the iterative solution of efficient on vector computers for  the iterative solution of linear systems.  Hence,   SLAP changes   this  input    data linear systems.  Hence,   SLAP changes   this  input    data structure to the SLAP Column format  for  the iteration (but structure to the SLAP Column format  for  the iteration (but does not change it back). does not change it back). Here is an example of the  SLAP Triad   storage format for a Here is an example of the  SLAP Triad   storage format for a 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 1  2  3  4  5  6  7  8  9 10 11 1  2  3  4  5  6  7  8  9 10 11 zeros in the matrix. Here is an example of the  SLAP Column  storage format for a |
| 5 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | INOUT    Integer IA(NELT). 5  1  1  3  1  5  5  2  3  4  2 zero. The JA array holds the offsets into the IA, A arrays for the beginning of   each    column.    That  is,    IA(JA(ICOL)), 1),  A(JA(ICOL+1)-1) points to the 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 \|  2  1 \|  3  5 \|  4 \|  5  1  3 is modified internally to be the SLAP Column format.  See above. Cautions: This routine will attempt to write to the Fortran logical output |
| 6 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | INOUT    Integer JA(NELT). 1  2  1  3  5  3  5  2  5  4  1 \| 0  0  0 44  0\| \|51  0 53  0 55\| =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the real array A.  In other words, for each column in the matrix th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have NELT+1, where  N  is the number of columns in  the 1  4  6    8  9   12 \| 0  0  0 44  0\| \|51  0 53  0 55\| Side Effects: is modified internally to be the SLAP Column format.  See above. Cautions: This routine will attempt to write to the Fortran logical output |
| 7 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NELT) | b  using the incomplete LU decomposition with iterative refinement. INOUT    Real A(NELT). These arrays should hold the matrix A in either the SLAP Triad format or the SLAP Column format.  See "Description", below.  If the SLAP Triad format is chosen it is changed internally to the SLAP Column format. 51 12 11 33 15 53 55 22 35 44 21 zero elements going down   the  column (except  the diagonal)  in th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 \| 22 12 \| 33 53 \| 44 \| 55 15 35 is modified internally to be the SLAP Column format.  See above. Cautions: This routine will attempt to write to the Fortran logical output zero value for IUNIT.  This routine does zero IUNIT unit number. |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. |
| 9 | `ITOL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Flag to indicate type of convergence criterion. norm of the residual divided by the 2-norm of the right-hand side is less than TOL. norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand 11 is often useful for checking and comparing different routines.  For this case, the user must supply the "exact" solution or a very accurate approximation (one with an error much less than TOL) through a common block, COMMON /SSLBLK/ SOLN( ) norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL.  Note that this requires the user to set up the "COMMON /SSLBLK/ SOLN(LENGTH)" in the calling routine. The routine with this declaration should be loaded before the stop test so that the correct length is used by the loader. This procedure is not standard Fortran and may not work correctly on your system (although it has worked on every system the authors have tried).  If ITOL is not 11 then this common block is indeed standard Fortran. |
| 10 | `TOL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | inv is the inverse of the diagonal of A. INOUT    Real. |
| 11 | `ITMAX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Maximum number of iterations. if convergence criterion could not be achieved in iterations. |
| 12 | `ITER` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | OUT      Integer. Number of iterations required to reach convergence, or |
| 13 | `ERR` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | OUT      Real. |
| 14 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 4.) OUT      Integer. Return error flag. 0 => All went well. 1 => Insufficient space allocated for WORK or IWORK. 2 => Method failed to converge in ITMAX steps. 3 => Error in user input. Check input values of N, ITOL. 4 => User error tolerance set too tight. Reset to 500*R1MACH(3).  Iteration proceeded. 5 => Preconditioning matrix, M, is not positive definite.  (r,z) < 0. 6 => Matrix A is not positive definite.  (p,Ap) < 0. 7 => Incomplete factorization broke down and was fudged.  Resulting preconditioning may be less than the best. |
| 15 | `IUNIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence.  If unit number is 0, no writing will occur. must make sure that must make sure that this logical unit is attached to a file or terminal before calling this logical unit is attached to a file or terminal before calling |
| 16 | `RWORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (LENW) | WORK     Real RWORK(LENW). Real array used for workspace. NL+NU+4*N. NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal). NU is the number of non-zeros in the upper triangle of the matrix (including the diagonal). |
| 17 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. NL+NU+4*N. NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal). NU is the number of non-zeros in the upper triangle of the matrix (including the diagonal). |
| 18 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (LENIW) | WORK     Integer IWORK(LENIW). Integer array used for workspace. Upon return the following locations of IWORK hold information which may be of use to the user: Amount of Integer workspace actually used. Amount of Real workspace actually used. NL+NU+4*N+10. NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal). NU is the number of non-zeros in the upper triangle of the matrix (including the diagonal). |
| 19 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. NL+NU+4*N+10. NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal). NU is the number of non-zeros in the upper triangle of the matrix (including the diagonal). |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

defined by ITOL.

### Storage and workspace requirements

`RWORK`: WORK     Real RWORK(LENW). Real array used for workspace. NL+NU+4*N. NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal). NU is the number of non-zeros in the upper triangle of the matrix (including the diagonal).

`IWORK`: WORK     Integer IWORK(LENIW). Integer array used for workspace. Upon return the following locations of IWORK hold information which may be of use to the user: Amount of Integer workspace actually used. Amount of Real workspace actually used. NL+NU+4*N+10. NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal). NU is the number of non-zeros in the upper triangle of the matrix (including the diagonal).

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::sparse::ssilur`. Native symbol: `ssilur_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::sparse::ssilur`
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
