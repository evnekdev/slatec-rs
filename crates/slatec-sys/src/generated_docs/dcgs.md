# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, IWORK(USER DEFINED) DOUBLE PRECISION B(N), X(N), A(NELT), TOL, ERR, R(N), R0(N), P(N) DOUBLE PRECISION Q(N), U(N), V1(N), V2(N), RWORK(USER DEFINED) EXTERNAL MATVEC, MSOLVE CALL DCGS(N, B, X, NELT, IA, JA, A, ISYM, MATVEC, $ MSOLVE, ITOL, TOL, ITMAX, ITER, ERR, IERR, IUNIT, $ R, R0, P, Q, U, V1, V2, RWORK, IWORK) This routine does not care what matrix data structure is used for A and M. It simply calls the MATVEC and MSOLVE routines, with the arguments as described above. The user could write any type of structure and the appropriate MATVEC and MSOLVE routines. It is assumed that A is stored in the

# Description

This canonical unsafe binding exposes original SLATEC routine `DCGS`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DCGS](https://www.netlib.org/slatec/lin/dcgs.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer Order of the Matrix. hand side vector, and Z is the solution upon return.  NELT,  IA, JA, A NELT+1, where N is the number of columns not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `B`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). IN       Double Precision B(N). Right-hand side vector. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `X`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). INOUT    Double Precision X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution. B. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `NELT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Number of Non-Zeros stored in A. zeros in the zeros in the matrix:  (IA(NELT), JA(NELT),  A(NELT)).  For each  non-zero matrix:  (IA(NELT), JA(NELT),  A(NELT)).  For each  non-zero the  user puts   the row  and  column index   of that matrix the  user puts   the row  and  column index   of that matrix zeros  in the matrix. Here is an example of the  SLAP Column  storage format for a not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `IA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). IN       Integer IA(NELT). JA, A  arrays in some fashion and  that M (or INV(M)) is stored  in  IWORK  and  RWORK   in  some fashion.   The SLAP routines DSDBCG and DSLUCS are examples of this procedure. Two  examples  of  matrix  data structures  are the: 1) SLAP Triad  format and 2) SLAP Column format. =================== S L A P Triad format =================== In  this   format only the  non-zeros are  stored.  They may appear  in *ANY* order.   The user  supplies three arrays of zero matrix  element is  placed in  the corresponding location of the A  array.  This is  an extremely easy data  structure to generate.  On  the other hand it  is  not too  efficient  on vector  computers   for the  iterative  solution  of  linear systems.  Hence, SLAP  changes this input  data structure to the SLAP   Column  format for the  iteration (but   does not change it back). Here is an example of the  SLAP Triad   storage format for a 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 1  2  3  4  5  6  7  8  9 10 11 5  1  1  3  1  5  5  2  3  4  2 1), A(JA(ICOL+1)-1) 1), A(JA(ICOL+1)-1) are  the last elements of the ICOL-th column.   Note that we are  the last elements of the ICOL-th column.   Note that we denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 |  2  1 |  3  5 |  4 |  5  1  3 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `JA`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (NELT). IN       Integer JA(NELT). zero matrix  element is  placed in  the corresponding location of the A  array.  This is  an extremely easy data  structure to generate.  On  the other hand it  is  not too  efficient  on vector  computers   for the  iterative  solution  of  linear systems.  Hence, SLAP  changes this input  data structure to the SLAP   Column  format for the  iteration (but   does not change it back). Here is an example of the  SLAP Triad   storage format for a 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 1  2  3  4  5  6  7  8  9 10 11 1  2  1  3  5  3  5  2  5  4  1 | 0  0  0 44  0| |51  0 53  0 55| =================== S L A P Column format ================== In  this format   the non-zeros are    stored counting  down columns (except  for the diagonal  entry, which must  appear first  in each "column") and are  stored in the  double pre- cision array  A. In  other  words,  for each  column  in the matrix  first put  the diagonal entry in A.  Then put in the other non-zero  elements going  down the column  (except the diagonal)  in order.  The IA array  holds the  row index for each non-zero.  The JA array  holds the offsets into the IA, 1), A(JA(ICOL+1)-1) are  the last elements of the ICOL-th column.   Note that we NELT+1, where N is the number of columns 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| Cautions: This routine will attempt to write to the Fortran logical output IN       Integer JA(NELT). zero matrix  element is  placed in  the corresponding location of the A  array.  This is  an extremely easy data  structure to generate.  On  the other hand it  is  not too  efficient  on vector  computers   for the  iterative  solution  of  linear systems.  Hence, SLAP  changes this input  data structure to the SLAP   Column  format for the  iteration (but   does not change it back). Here is an example of the  SLAP Triad   storage format for a 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 1  2  3  4  5  6  7  8  9 10 11 1  2  1  3  5  3  5  2  5  4  1 | 0  0  0 44  0| |51  0 53  0 55| =================== S L A P Column format ================== In  this format   the non-zeros are    stored counting  down columns (except  for the diagonal  entry, which must  appear first  in each "column") and are  stored in the  double pre- cision array  A. In  other  words,  for each  column  in the matrix  first put  the diagonal entry in A.  Then put in the other non-zero  elements going  down the column  (except the diagonal)  in order.  The IA array  holds the  row index for each non-zero.  The JA array  holds the offsets into the IA, 1), A(JA(ICOL+1)-1) are  the last elements of the ICOL-th column.   Note that we NELT+1, where N is the number of columns 1  4  6    8  9   12 | 0  0  0 44  0| |51  0 53  0 55| Cautions: This routine will attempt to write to the Fortran logical output not applicable or not stated by selected source not a workspace argument

## 7. `A`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (NELT). Symmetric linear system  Ax = b using the Preconditioned BiConjugate Gradient Squared IN       Double Precision A(NELT). These arrays contain the matrix data structure for A. It could take any form.  See "Description", below, for more details. R  for Z R  for Z given R with the preconditioning matrix M (M is supplied via given R with the preconditioning matrix M (M is supplied via B. 51 12 11 33 15 53 55 22 35 44 21 arrays  for  the  beginning  of  each  column.  That  is, 1), A(JA(ICOL+1)-1) are  the last elements of the ICOL-th column.   Note that we denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 zero value for IUNIT.  This routine does zero IUNIT unit number. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `ISYM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. see Description, below.  RWORK is a  double precision array that can be used to pass necessary preconditioning information and/ not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `MATVEC`

callback `callback` argument; Fortran declaration `INTEGER`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. EXT      External. Name of a routine which  performs the matrix vector multiply operation  Y = A*X  given A and X.  The  name of  the MATVEC routine must  be declared external  in the  calling program. CALL MATVEC( N, X, Y, NELT, IA, JA, A, ISYM ) Where N is the number of unknowns, Y is the product A*X upon return,  X is an input  vector.  NELT, IA,  JA,  A and  ISYM define the SLAP matrix data structure: see Description,below. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. EXT      External. Name of a routine which  performs the matrix vector multiply operation  Y = A*X  given A and X.  The  name of  the MATVEC routine must  be declared external  in the  calling program. CALL MATVEC( N, X, Y, NELT, IA, JA, A, ISYM ) Where N is the number of unknowns, Y is the product A*X upon return,  X is an input  vector.  NELT, IA,  JA,  A and  ISYM define the SLAP matrix data structure: see Description,below. not applicable or not stated by selected source not a workspace argument

## 10. `MSOLVE`

callback `callback` argument; Fortran declaration `INTEGER`, Rust ABI type `reviewed unsafe extern callback function pointer`, and scalar. EXT      External. CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) is an integer work array for the same purpose as RWORK. The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran. not stated by selected source not applicable or not stated by selected source

## 11. `ITOL`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Flag to indicate type of convergence criterion. norm of the residual divided by the 2-norm of the right-hand side is less than TOL. 2 is *STRONGLY* recommended. norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand 11 is often useful for checking and comparing different routines.  For this case, the user must supply the "exact" solution or a very accurate approximation (one with an error much less than TOL) through a common block, COMMON /DSLBLK/ SOLN( ) norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL. IN       Integer. Flag to indicate type of convergence criterion. norm of the residual divided by the 2-norm of the right-hand side is less than TOL. 2 is *STRONGLY* recommended. norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand 11 is often useful for checking and comparing different routines.  For this case, the user must supply the "exact" solution or a very accurate approximation (one with an error much less than TOL) through a common block, COMMON /DSLBLK/ SOLN( ) norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL. not applicable or not stated by selected source not a workspace argument

## 12. `TOL`

input-output `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. inv time a vector is the pre- conditioning step.  This is the *NATURAL* stopping for this iterative method and is *STRONGLY* recommended. INOUT    Double Precision. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 13. `ITMAX`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Maximum number of iterations. if convergence criterion could not be achieved in iterations. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 14. `ITER`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. OUT      Integer. Number of iterations required to reach convergence, or not stated by selected source not applicable or not stated by selected source not a workspace argument

## 15. `ERR`

input `scalar` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and scalar. OUT      Double Precision. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 16. `IERR`

input-output `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. 4.) OUT      Integer. Return error flag. 0 => All went well. 1 => Insufficient space allocated for WORK or IWORK. 2 => Method failed to converge in ITMAX steps. 3 => Error in user input. Check input values of N, ITOL. 4 => User error tolerance set too tight. Reset to 500*D1MACH(3).  Iteration proceeded. 5 => Breakdown of the method detected. 6 => Stagnation of the method detected. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 17. `IUNIT`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN       Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence.  If unit number is 0, no writing will occur. must make sure that must make sure that this logical unit is attached to a file or terminal before calling this logical unit is attached to a file or terminal before calling IN       Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence.  If unit number is 0, no writing will occur. must make sure that must make sure that this logical unit is attached to a file or terminal before calling this logical unit is attached to a file or terminal before calling not applicable or not stated by selected source not a workspace argument

## 18. `R`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). hand side vector, and Z is the solution upon return.  NELT,  IA, JA, A B. approximately 0. WORK     Double Precision R(N). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 19. `R0`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). approximately 0. approximately 0. WORK     Double Precision R0(N). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 20. `P`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). WORK     Double Precision P(N). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 21. `Q`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). WORK     Double Precision Q(N). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 22. `U`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). WORK     Double Precision U(N). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 23. `V1`

input-output `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). WORK     Double Precision V1(N). not stated by selected source not applicable or not stated by selected source not a workspace argument

## 24. `V2`

input `array` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (N). WORK     Double Precision V2(N). Double Precision arrays used for workspace. not stated by selected source not applicable or not stated by selected source

## 25. `RWORK`

workspace `workspace` argument; Fortran declaration `DOUBLE PRECISION`, Rust ABI type `*mut f64`, and rank 1; dimensions (*). and IWORK arrays).   The name  of  the MSOLVE routine must be declared  external  in the  calling   program.   The WORK     Double Precision RWORK(USER DEFINED). Double Precision array that can be used for workspace in and IWORK arrays).   The name  of  the MSOLVE routine must be declared  external  in the  calling   program.   The WORK     Double Precision RWORK(USER DEFINED). Double Precision array that can be used for workspace in not applicable or not stated by selected source

## 26. `IWORK`

workspace `workspace` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). is an integer work array for the same purpose as RWORK. WORK     Integer IWORK(USER DEFINED). Integer array that can be used for workspace in MSOLVE. not stated by selected source not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Callback arguments use the reviewed ABI shown by their Rust function-pointer type. They are invoked synchronously by the native call, must remain valid until it returns, must uphold every documented input/output extent, and **must not unwind** through Fortran. A callback must not retain or free caller-owned native buffers unless the source contract expressly permits it.

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
- `MATVEC`: not a workspace argument
- `MSOLVE`: EXT      External. CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) is an integer work array for the same purpose as RWORK.
- `ITOL`: not a workspace argument
- `TOL`: not a workspace argument
- `ITMAX`: not a workspace argument
- `ITER`: not a workspace argument
- `ERR`: not a workspace argument
- `IERR`: not a workspace argument
- `IUNIT`: not a workspace argument
- `R`: not a workspace argument
- `R0`: not a workspace argument
- `P`: not a workspace argument
- `Q`: not a workspace argument
- `U`: not a workspace argument
- `V1`: not a workspace argument
- `V2`: WORK     Double Precision V2(N). Double Precision arrays used for workspace.
- `RWORK`: and IWORK arrays).   The name  of  the MSOLVE routine must be declared  external  in the  calling   program.   The WORK     Double Precision RWORK(USER DEFINED). Double Precision array that can be used for workspace in
- `IWORK`: is an integer work array for the same purpose as RWORK. WORK     Integer IWORK(USER DEFINED). Integer array that can be used for workspace in MSOLVE.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::dcgs`
- Original SLATEC routine: `DCGS`
- Native symbol: `dcgs_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [DCGS](https://www.netlib.org/slatec/lin/dcgs.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
