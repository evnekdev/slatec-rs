# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, LENW, IWORK(NL+2*N+1), LENIW REAL B(N), X(N), A(NELT), TOL, ERR, RWORK(NL+5*N) CALL SSICCG(N, B, X, NELT, IA, JA, A, ISYM, ITOL, TOL, $ ITMAX, ITER, ERR, IERR, IUNIT, RWORK, LENW, IWORK, LENIW ) This routine performs preconditioned conjugate gradient method on the symmetric positive definite linear system

# Description

This canonical unsafe binding exposes original SLATEC routine `SSICCG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSICCG](https://www.netlib.org/slatec/lin/ssiccg.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Order of the Matrix. NELT+1, where  N  is the number of columns in  the not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (N). IN       Real B(N). Right-hand side vector. The preconditioner  is  the incomplete Cholesky (IC) factorization of the matrix A.  See  SSICS for details about the incomplete   factorization algorithm.  One   should note here however, that the  IC factorization is a  slow  process and  that  one should   save  factorizations  for  reuse, if possible.  The   MSOLVE operation (handled  in  SSLLTI) does vectorize on machines  with  hardware  gather/scatter and is quite fast. The Sparse Linear Algebra Package (SLAP) utilizes two matrix data structures: 1) the  SLAP Triad  format or  2)  the SLAP Column format.  The user can hand this routine either of the of these data structures and SLAP  will figure out  which on is being used and act accordingly. =================== S L A P Triad format =================== This routine requires that the  matrix A be   stored in  the SLAP  Triad format.  In  this format only the non-zeros  are stored.  They may appear in  *ANY* order.  The user supplies not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (N). INOUT    Real X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `NELT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Number of Non-Zeros stored in A. is  the number  of is  the number  of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)).  For non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)).  For each non-zero the user puts the row and column index of that each non-zero the user puts the row and column index of that matrix element  in the IA and  JA arrays.  The  value of the matrix element  in the IA and  JA arrays.  The  value of the non-zero   matrix  element is  placed  in  the corresponding non-zero   matrix  element is  placed  in  the corresponding location of the A array.   This is  an  extremely  easy data location of the A array.   This is  an  extremely  easy data structure to generate.  On  the  other hand it   is  not too structure to generate.  On  the  other hand it   is  not too efficient on vector computers for  the iterative solution of efficient on vector computers for  the iterative solution of linear systems.  Hence,   SLAP changes   this  input    data linear systems.  Hence,   SLAP changes   this  input    data structure to the SLAP Column format  for  the iteration (but structure to the SLAP Column format  for  the iteration (but does not change it back). does not change it back). Here is an example of the  SLAP Triad   storage format for a Here is an example of the  SLAP Triad   storage format for a 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 1  2  3  4  5  6  7  8  9 10 11 1  2  3  4  5  6  7  8  9 10 11 zeros in the matrix. Here is an example of the  SLAP Column  storage format for a not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `IA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). INOUT    Integer IA(NELT). 5  1  1  3  1  5  5  2  3  4  2 zero. The JA array holds the offsets into the IA, A arrays for the beginning of   each    column.    That  is,    IA(JA(ICOL)), 1),  A(JA(ICOL+1)-1) points to the 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 |  2  1 |  3  5 |  4 |  5  1  3 is modified internally to be the SLAP Column format.  See above. Cautions: This routine will attempt to write to the Fortran logical output not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `JA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). INOUT    Integer JA(NELT). 1  2  1  3  5  3  5  2  5  4  1 | 0  0  0 44  0| |51  0 53  0 55| =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the real array A.  In other words, for each column in the matrix th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have NELT+1, where  N  is the number of columns in  the 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| Side Effects: is modified internally to be the SLAP Column format.  See above. Cautions: This routine will attempt to write to the Fortran logical output INOUT    Integer JA(NELT). 1  2  1  3  5  3  5  2  5  4  1 | 0  0  0 44  0| |51  0 53  0 55| =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the real array A.  In other words, for each column in the matrix th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have NELT+1, where  N  is the number of columns in  the 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| Side Effects: is modified internally to be the SLAP Column format.  See above. Cautions: This routine will attempt to write to the Fortran logical output not applicable or not stated by selected source not a workspace argument

## 7. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (NELT). INOUT    Real A(NELT). These arrays should hold the matrix A in either the SLAP Triad format or the SLAP Column format.  See "Description", below.  If the SLAP Triad format is chosen it is changed internally to the SLAP Column format. 51 12 11 33 15 53 55 22 35 44 21 zero elements going down   the  column (except  the diagonal)  in th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 is modified internally to be the SLAP Column format.  See above. Cautions: This routine will attempt to write to the Fortran logical output zero value for IUNIT.  This routine does zero IUNIT unit number. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `ISYM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `ITOL`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Flag to indicate type of convergence criterion. norm of the residual divided by the 2-norm of the right-hand side is less than TOL. norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand 11 is often useful for checking and comparing different routines.  For this case, the user must supply the "exact" solution or a very accurate approximation (one with an error much less than TOL) through a common block, COMMON /SSLBLK/ SOLN( ) norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL.  Note that this requires the user to set up the "COMMON /SSLBLK/ SOLN(LENGTH)" in the calling routine. The routine with this declaration should be loaded before the stop test so that the correct length is used by the loader. This procedure is not standard Fortran and may not work correctly on your system (although it has worked on every system the authors have tried).  If ITOL is not 11 then this common block is indeed standard Fortran. IN       Integer. Flag to indicate type of convergence criterion. norm of the residual divided by the 2-norm of the right-hand side is less than TOL. norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand 11 is often useful for checking and comparing different routines.  For this case, the user must supply the "exact" solution or a very accurate approximation (one with an error much less than TOL) through a common block, COMMON /SSLBLK/ SOLN( ) norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL.  Note that this requires the user to set up the "COMMON /SSLBLK/ SOLN(LENGTH)" in the calling routine. The routine with this declaration should be loaded before the stop test so that the correct length is used by the loader. This procedure is not standard Fortran and may not work correctly on your system (although it has worked on every system the authors have tried).  If ITOL is not 11 then this common block is indeed standard Fortran. not applicable or not stated by selected source not a workspace argument

## 10. `TOL`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. inv is the inverse of the diagonal of A. INOUT    Real. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 11. `ITMAX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Maximum number of iterations. if convergence criterion could not be achieved in iterations. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 12. `ITER`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. OUT      Integer. Number of iterations required to reach convergence, or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `ERR`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. OUT      Real. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 14. `IERR`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 4.) OUT      Integer. Return error flag. 0 => All went well. 1 => Insufficient space allocated for WORK or IWORK. 2 => Method failed to converge in ITMAX steps. 3 => Error in user input. Check input values of N, ITOL. 4 => User error tolerance set too tight. Reset to 500*R1MACH(3).  Iteration proceeded. 5 => Preconditioning matrix, M, is not positive definite.  (r,z) < 0. 6 => Matrix A is not positive definite.  (p,Ap) < 0. 7 => Incomplete factorization broke down and was fudged.  Resulting preconditioning may be less than the best. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 15. `IUNIT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence.  If unit number is 0, no writing will occur. must make sure that must make sure that this logical unit is attached to a file or terminal before calling this logical unit is attached to a file or terminal before calling IN       Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence.  If unit number is 0, no writing will occur. must make sure that must make sure that this logical unit is attached to a file or terminal before calling this logical unit is attached to a file or terminal before calling not applicable or not stated by selected source not a workspace argument

## 16. `RWORK`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (LENW). WORK     Real RWORK(LENW). Real array used for workspace. NL+5*N. NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal). not stated by selected source not applicable or not stated by selected source

## 17. `LENW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. NL+5*N. NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 18. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (LENIW). WORK     Integer IWORK(LENIW). Integer array used for workspace. Upon return the following locations of IWORK hold information which may be of use to the user: Amount of Integer workspace actually used. Amount of Real workspace actually used. NL+N+11. NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal). not stated by selected source not applicable or not stated by selected source

## 19. `LENIW`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. NL+N+11. NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal). not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

defined by ITOL.

# Workspace and array requirements

- `N`: not a workspace argument
- `B`: not a workspace argument
- `X`: not a workspace argument
- `NELT`: not a workspace argument
- `IA`: not a workspace argument
- `JA`: not a workspace argument
- `A`: not a workspace argument
- `ISYM`: not a workspace argument
- `ITOL`: not a workspace argument
- `TOL`: not a workspace argument
- `ITMAX`: not a workspace argument
- `ITER`: not a workspace argument
- `ERR`: not a workspace argument
- `IERR`: not a workspace argument
- `IUNIT`: not a workspace argument
- `RWORK`: WORK     Real RWORK(LENW). Real array used for workspace. NL+5*N. NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal).
- `LENW`: not a workspace argument
- `IWORK`: WORK     Integer IWORK(LENIW). Integer array used for workspace. Upon return the following locations of IWORK hold information which may be of use to the user: Amount of Integer workspace actually used. Amount of Real workspace actually used. NL+N+11. NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal).
- `LENIW`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::sparse::ssiccg`
- Original SLATEC routine: `SSICCG`
- Native symbol: `ssiccg_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [SSICCG](https://www.netlib.org/slatec/lin/ssiccg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
