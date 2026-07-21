# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, LENW, IWORK(10), LENIW REAL B(N), X(N), A(NELT), TOL, ERR, RWORK(8*N) CALL SSDCGS(N, B, X, NELT, IA, JA, A, ISYM, ITOL, TOL, $ ITMAX, ITER, ERR, IERR, IUNIT, RWORK, LENW, IWORK, LENIW ) This routine performs preconditioned BiConjugate gradient method on the Non-Symmetric positive definite linear system Ax=b. The preconditioner is M = DIAG(A), the diagonal of the matrix A. This is the simplest of preconditioners and vectorizes very well. The Sparse Linear Algebra Package (SLAP) utilizes two matrix data structures: 1) the SLAP Triad format or 2) the SLAP Column format. The user can hand this routine either of the of these data structures and SLAP will figure out which on is being used and act accordingly. =================== S L A P Triad format =================== This routine requires that the matrix A be stored in the SLAP Triad format. In this format only the non-zeros are stored. They may appear in *ANY* order. The user supplies three arrays of length NELT, where NELT is the number of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)). For each non-zero the user puts the row and column index of that matrix element in the IA and JA arrays. The value of the non-zero matrix element is placed in the corresponding location of the A array. This is an extremely easy data structure to generate. On the other hand it is not too efficient on vector computers for the iterative solution of linear systems. Hence, SLAP changes this input data structure to the SLAP Column format for the iteration (but does not change it back). Here is an example of the SLAP Triad storage format for a 5x5 Matrix. Recall that the entries may appear in any order. 5x5 Matrix SLAP Triad format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 51 12 11 33 15 53 55 22 35 44 21 |21 22 0 0 0| IA: 5 1 1 3 1 5 5 2 3 4 2 | 0 0 33 0 35| JA: 1 2 1 3 5 3 5 2 5 4 1 | 0 0 0 44 0| |51 0 53 0 55| =================== S L A P Column format ================== SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the end of the ICOL-th column. Note that we always have

# Description

This canonical unsafe binding exposes original SLATEC routine `SSDCGS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSDCGS](https://www.netlib.org/slatec/lin/ssdcgs.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Order of the Matrix.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

B(N). Right-hand side vector.

## `X`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution.

## `NELT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Number of Non-Zeros stored in A.

## `IA`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

IA(NELT).

## `JA`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

JA(NELT). NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 |21 22 0 0 0| IA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| JA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| Side Effects: The SLAP Triad format (IA, JA, A) is modified internally to be the SLAP Column format. See above. Cautions: This routine will attempt to write to the Fortran logical output unit IUNIT, if IUNIT.

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

A(NELT). These arrays should hold the matrix A in either the SLAP Triad format or the SLAP Column format. See "Description", below. If the SLAP Triad format is chosen it is changed internally to the SLAP Column format.

## `ISYM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored.

## `ITOL`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Flag to indicate type of convergence criterion. If ITOL=1, iteration stops when the 2-norm of the residual divided by the 2-norm of the right-hand side is less than TOL. This routine must calculate the residual from R = A*X - B. This is unnatural and hence expensive for this type of iter- ative method. ITOL=2 is *STRONGLY* recommended.

## `TOL`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Real. Convergence criterion, as described above. (Reset if IERR=4. ).

## `ITMAX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Maximum number of iterations.

## `ITER`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Number of iterations required to reach convergence, or ITMAX+1 if convergence criterion could not be achieved in ITMAX iterations.

## `ERR`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Real. Error estimate of error in final approximate solution, as defined by ITOL.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Return error flag. 0 => All went well. 1 => Insufficient space allocated for WORK or IWORK. 2 => Method failed to converge in ITMAX steps. 3 => Error in user input.

## `IUNIT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence. If unit number is 0, no writing will occur.

## `RWORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (LENW).

RWORK(LENW). array used for workspace.

## `LENW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Length of the real workspace, RWORK. LENW >= 8*N.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (LENIW).

IWORK(LENIW). Used to hold pointers into the RWORK array. Upon return the following locations of IWORK hold information which may be of use to the user: Amount of Integer workspace actually used. IWORK(10) Amount of Real workspace actually used.

## `LENIW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Length of the integer workspace, IWORK. LENIW >= 10.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `B`: not a workspace argument
- `X`: not a workspace argument
- `IA`: not a workspace argument
- `JA`: not a workspace argument
- `A`: not a workspace argument
- `RWORK`: RWORK(LENW). array used for workspace.
- `IWORK`: IWORK(LENIW). Used to hold pointers into the RWORK array. Upon return the following locations of IWORK hold information which may be of use to the user: Amount of Integer workspace actually used. IWORK(10) Amount of Real workspace actually used.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::sparse::ssdcgs`
- Original SLATEC routine: `SSDCGS`
- Native symbol: `ssdcgs_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [SSDCGS](https://www.netlib.org/slatec/lin/ssdcgs.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
