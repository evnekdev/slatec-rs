# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, NSAVE, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, IWORK(USER DEFINED) DOUBLE PRECISION B(N), X(N), A(NELT), TOL, ERR, R(N), Z(N) DOUBLE PRECISION P(N,0:NSAVE), AP(N,0:NSAVE), EMAP(N,0:NSAVE) DOUBLE PRECISION DZ(N), CSAV(NSAVE), RWORK(USER DEFINED) EXTERNAL MATVEC, MSOLVE CALL DOMN(N, B, X, NELT, IA, JA, A, ISYM, MATVEC, MSOLVE, $ NSAVE, ITOL, TOL, ITMAX, ITER, ERR, IERR, IUNIT, R, $ Z, P, AP, EMAP, DZ, CSAV, RWORK, IWORK) This routine does not care what matrix data structure is used for A and M. It simply calls the MATVEC and MSOLVE routines, with the arguments as described above. The user could write any type of structure and the appropriate MATVEC and MSOLVE routines. It is assumed that A is stored in the IA, JA, A arrays in some fashion and that M (or INV(M)) is stored in IWORK and RWORK) in some fashion. The SLAP routines DSDOMN and DSLUOM are examples of this procedure. Two examples of matrix data structures are the: 1) SLAP Triad format and 2) SLAP Column format. =================== S L A P Triad format =================== In this format only the non-zeros are stored. They may appear in *ANY* order. The user supplies three arrays of length NELT, where NELT is the number of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)). For each non-zero the user puts the row and column index of that matrix element in the IA and JA arrays. The value of the non-zero matrix element is placed in the corresponding location of the A array. This is an extremely easy data structure to generate. On the other hand it is not too efficient on vector computers for the iterative solution of linear systems. Hence, SLAP changes this input data structure to the SLAP Column format for the iteration (but does not change it back). Here is an example of the SLAP Triad storage format for a 5x5 Matrix. Recall that the entries may appear in any order. 5x5 Matrix SLAP Triad format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 51 12 11 33 15 53 55 22 35 44 21 |21 22 0 0 0| IA: 5 1 1 3 1 5 5 2 3 4 2 | 0 0 33 0 35| JA: 1 2 1 3 5 3 5 2 5 4 1 | 0 0 0 44 0| |51 0 53 0 55| =================== S L A P Column format ================== In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the double pre- cision array A. In other words, for each column in the matrix first put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA,

# Description

This canonical unsafe binding exposes original SLATEC routine `DOMN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DOMN](https://www.netlib.org/slatec/lin/domn.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Order of the Matrix.

## `B`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (N).

Double Precision B(N). Right-hand side vector.

## `X`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (N).

Double Precision X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution.

## `NELT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Number of Non-Zeros stored in A.

## `IA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

IA(NELT). the first elements of the ICOL- th column in IA and A, and IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) are the last elements of the ICOL-th column. Note that we always have JA(N+1)=NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 |21 22 0 0 0| IA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| JA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| Cautions: This routine will attempt to write to the Fortran logical output unit IUNIT, if IUNIT. ne.

## `JA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

JA(NELT).

## `A`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (NELT).

Double Precision A(NELT). These arrays contain the matrix data structure for A. It could take any form. See "Description", below, for more details. arrays for the beginning of each column. That is, the first elements of the ICOL- th column in IA and A, and IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) are the last elements of the ICOL-th column.

## `ISYM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. is a flag which, if non-zero, denotest that A is symmetric and only the lower or upper triangle is stored. are defined as above.

## `MATVEC`

**Direction:** `callback`. **Fortran type:** `INTEGER`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

External. Name of a routine which performs the matrix vector multiply Y = A*X given A and X. The name of the MATVEC routine must be declared external in the calling program. The calling sequence to MATVEC is: CALL MATVEC( N, X, Y, NELT, IA, JA, A, ISYM ) Where N is the number of unknowns, Y is the product A*X upon return X is an input vector, NELT is the number of non-zeros in the SLAP IA, JA, A storage for the matrix A. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `MSOLVE`

**Direction:** `callback`. **Fortran type:** `INTEGER`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

External. Name of a routine which solves a linear system MZ = R for Z given R with the preconditioning matrix M (M is supplied via RWORK and IWORK arrays). The name of the MSOLVE routine must be declared external in the calling program. The calling sequence to MSOLVE is: CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) Where N is the number of unknowns, R is the right-hand side vector and Z is the solution upon return. NELT, IA, JA, A and. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `NSAVE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Number of direction vectors to save and orthogonalize against. NSAVE >= 0.

## `ITOL`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Flag to indicate type of convergence criterion. If ITOL=1, iteration stops when the 2-norm of the residual divided by the 2-norm of the right-hand side is less than TOL. If ITOL=2, iteration stops when the 2-norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand side is less than TOL, where M-inv is the inverse of the diagonal of A. 11 is often useful for checking and comparing different routines. For this case, the user must supply the "exact" solution or a very accurate approximation (one with an error much less than TOL) through a common block, COMMON /DSLBLK/ SOLN( ) If ITOL=11, iteration stops when the 2-norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL.

## `TOL`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Double Precision. Convergence criterion, as described above. (Reset if IERR=4. ).

## `ITMAX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Maximum number of iterations.

## `ITER`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Number of iterations required to reach convergence, or ITMAX+1 if convergence criterion could not be achieved in ITMAX iterations.

## `ERR`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Double Precision. Error estimate of error in final approximate solution, as defined by ITOL.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Return error flag. 0 => All went well. 1 => Insufficient space allocated for WORK or IWORK. 2 => Method failed to converge in ITMAX steps. 3 => Error in user input.

## `IUNIT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence. If unit number is 0, no writing will occur.

## `R`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (N).

Double Precision R(N).

## `Z`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (N).

Double Precision Z(N).

## `P`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (N, 0:NSAVE).

Double Precision P(N,0:NSAVE).

## `AP`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (N, 0:NSAVE).

Double Precision AP(N,0:NSAVE).

## `EMAP`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (N, 0:NSAVE).

Double Precision EMAP(N,0:NSAVE).

## `DZ`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (N).

Double Precision DZ(N).

## `CSAV`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (NSAVE).

Double Precision CSAV(NSAVE) Double Precision arrays used for workspace.

## `RWORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Double Precision RWORK(USER DEFINED). Double Precision array that can be used for workspace in.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

IWORK(USER DEFINED). array that can be used for workspace in MSOLVE.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Workspace and array requirements

- `B`: not a workspace argument
- `X`: not a workspace argument
- `IA`: not a workspace argument
- `JA`: not a workspace argument
- `A`: not a workspace argument
- `R`: not a workspace argument
- `Z`: not a workspace argument
- `P`: not a workspace argument
- `AP`: not a workspace argument
- `EMAP`: not a workspace argument
- `DZ`: not a workspace argument
- `CSAV`: Double Precision CSAV(NSAVE) Double Precision arrays used for workspace.
- `RWORK`: Double Precision RWORK(USER DEFINED). Double Precision array that can be used for workspace in.
- `IWORK`: IWORK(USER DEFINED). array that can be used for workspace in MSOLVE.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::domn`
- Original SLATEC routine: `DOMN`
- Native symbol: `domn_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [DOMN](https://www.netlib.org/slatec/lin/domn.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
