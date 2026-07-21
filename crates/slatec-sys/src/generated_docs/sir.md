# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, IWORK(USER DEFINED) REAL B(N), X(N), A(NELT), TOL, ERR, R(N), Z(N), DZ(N), REAL RWORK(USER DEFINED) EXTERNAL MATVEC, MSOLVE CALL SIR(N, B, X, NELT, IA, JA, A, ISYM, MATVEC, MSOLVE, ITOL, $ TOL, ITMAX, ITER, ERR, IERR, IUNIT, R, Z, DZ, RWORK, IWORK) The basic algorithm for iterative refinement (also known as iterative improvement) is: n+1 n -1 n

# Description

This canonical unsafe binding exposes original SLATEC routine `SIR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SIR](https://www.netlib.org/slatec/lin/sir.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Order of the Matrix.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

B(N). Right-hand side vector.

## `X`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution. X + M (B - AX ). -1 -1 If M = A then this is the standard iterative refinement algorithm and the "subtraction" in the residual calculation should be done in double precision (which it is not in this routine). If M = DIAG(A), the diagonal of A, then iterative refinement is known as Jacobi's method.

## `NELT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Number of Non-Zeros stored in A. is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 |21 22 0 0 0| IA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| JA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| Examples: See the SLAP routines SSJAC, SSGS Cautions: This routine will attempt to write to the Fortran logical output unit IUNIT, if IUNIT. ne.

## `IA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

IA(NELT).

## `JA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

JA(NELT).

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (NELT).

A(NELT). These arrays contain the matrix data structure for A. It could take any form. See "Description", below, for more details.

## `ISYM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. is a flag which, if non-zero, denotes that A is symmetric and only the lower or upper triangle is stored. are defined as above.

## `MATVEC`

**Direction:** `callback`. **Fortran type:** `INTEGER`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

External. Name of a routine which performs the matrix vector multiply Y = A*X given A and X. The name of the MATVEC routine must be declared external in the calling program. The calling sequence to MATVEC is: CALL MATVEC( N, X, Y, NELT, IA, JA, A, ISYM ) Where N is the number of unknowns, Y is the product A*X upon return, X is an input vector, NELT is the number of non-zeros in the SLAP IA, JA, A storage for the matrix A. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `MSOLVE`

**Direction:** `callback`. **Fortran type:** `INTEGER`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

External. Name of a routine which solves a linear system MZ = R for Z given R with the preconditioning matrix M (M is supplied via RWORK and IWORK arrays). The name of the MSOLVE routine must be declared external in the calling program. The calling sequence to MSOLVE is: CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) Where N is the number of unknowns, R is the right-hand side vector and Z is the solution upon return. NELT, IA, JA, A and. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `ITOL`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Flag to indicate type of convergence criterion. If ITOL=1, iteration stops when the 2-norm of the residual divided by the 2-norm of the right-hand side is less than TOL. If ITOL=2, iteration stops when the 2-norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand side is less than TOL, where M-inv is the inverse of the diagonal of A. 11 is often useful for checking and comparing different routines. For this case, the user must supply the "exact" solution or a very accurate approximation (one with an error much less than TOL) through a common block, COMMON /SSLBLK/ SOLN( ) If ITOL=11, iteration stops when the 2-norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL.

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

## `R`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

R(N).

## `Z`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

Z(N).

## `DZ`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

DZ(N). arrays used for workspace.

## `RWORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

RWORK(USER DEFINED). array that can be used by MSOLVE. Real RWORK(USER DEFINED). Real array that can be used by MSOLVE.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

IWORK(USER DEFINED). array that can be used by MSOLVE. Integer IWORK(USER DEFINED). Integer array that can be used by MSOLVE.

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
- `DZ`: DZ(N). arrays used for workspace.
- `RWORK`: Real RWORK(USER DEFINED). Real array that can be used by MSOLVE.
- `IWORK`: Integer IWORK(USER DEFINED). Integer array that can be used by MSOLVE.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::sir`
- Original SLATEC routine: `SIR`
- Native symbol: `sir_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32),mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [SIR](https://www.netlib.org/slatec/lin/sir.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
