# Purpose

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, LRGW, IGWK(LIGW), LIGW INTEGER IWORK(USER DEFINED) REAL B(N), X(N), A(NELT), TOL, ERR, SB(N), SX(N) REAL RGWK(LRGW), RWORK(USER DEFINED) EXTERNAL MATVEC, MSOLVE CALL SGMRES(N, B, X, NELT, IA, JA, A, ISYM, MATVEC, MSOLVE, $ ITOL, TOL, ITMAX, ITER, ERR, IERR, IUNIT, SB, SX, $ RGWK, LRGW, IGWK, LIGW, RWORK, IWORK) SGMRES solves a linear system A*X = B rewritten in the form: (SB*A*(M-inverse)*(SX-inverse))*(SX*M*X) = SB*B, with right preconditioning, or (SB*(M-inverse)*A*(SX-inverse))*(SX*X) = SB*(M-inverse)*B, with left preconditioning, where A is an N-by-N real matrix,

# Description

This canonical unsafe binding exposes original SLATEC routine `SGMRES`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SGMRES](https://www.netlib.org/slatec/lin/sgmres.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Order of the Matrix.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

B(N). Right-hand side vector. N-vectors, SB and SX are diagonal scaling matrices, and M is a preconditioning matrix. It uses preconditioned Krylov subpace methods based on the generalized minimum residual method (GMRES). This routine optionally performs either the full orthogonalization version of the GMRES algorithm or an incomplete variant of it. Both versions use restarting of the linear iteration by default, although the user can disable this feature.

## `X`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

X(N). On input X is your initial guess for the solution vector. On output X is the final approximate solution. N-vectors, SB and SX are diagonal scaling matrices, and M is a preconditioning matrix. It uses preconditioned Krylov subpace methods based on the generalized minimum residual method (GMRES). This routine optionally performs either the full orthogonalization version of the GMRES algorithm or an incomplete variant of it.

## `NELT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Number of Non-Zeros stored in A.

## `IA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

IA(NELT).

## `JA`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (NELT).

JA(NELT). NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 |21 22 0 0 0| IA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| JA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| Cautions: This routine will attempt to write to the Fortran logical output unit IUNIT, if IUNIT. ne. 0.

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (NELT).

A(NELT). These arrays contain the matrix data structure for A. It could take any form. See "Description", below, for more details. The actual stopping test is either: norm(SB*(B-A*X(L))). le.

## `ISYM`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. is a flag which, if non-zero, denotes that A is symmetric and only the lower or upper triangle is stored. are defined as above.

## `MATVEC`

**Direction:** `callback`. **Fortran type:** `INTEGER`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

External. Name of a routine which performs the matrix vector multiply Y = A*X given A and X. The name of the MATVEC routine must be declared external in the calling program. The calling sequence to MATVEC is: CALL MATVEC(N, X, Y, NELT, IA, JA, A, ISYM) where N is the number of unknowns, Y is the product A*X upon return, X is an input vector, and NELT is the number of non-zeros in the SLAP IA, JA, A storage for the matrix A. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `MSOLVE`

**Direction:** `callback`. **Fortran type:** `INTEGER`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

External. Name of the routine which solves a linear system Mz = r for z given r with the preconditioning matrix M (M is supplied via RWORK and IWORK arrays. The name of the MSOLVE routine must be declared external in the calling program. The calling sequence to MSOLVE is: CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) Where N is the number of unknowns, R is the right-hand side vector and Z is the solution upon return. NELT, IA, JA, A and. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `ITOL`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Flag to indicate the type of convergence criterion used. 0 Means the iteration stops when the test described below on the residual RL is satisfied. This is the "Natural Stopping Criteria" for this routine. Other values of ITOL cause extra, otherwise unnecessary, computation per iteration and are therefore much less efficient. See ISSGMR (the stop test routine) for more information.

## `TOL`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Real. Convergence criterion, as described below. If TOL is set to zero on input, then a default value of 500*(the smallest positive magnitude, machine epsilon) is used.

## `ITMAX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

DUMMY Integer. Maximum number of iterations in most SLAP routines. In this routine this does not make sense. The maximum number of iterations here is given by ITMAX = MAXL*(NRMAX+1). See IGWK for definitions of MAXL and NRMAX.

## `ITER`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Number of iterations required to reach convergence, or ITMAX if convergence criterion could not be achieved in ITMAX iterations.

## `ERR`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Real. Error estimate of error in final approximate solution, as defined by ITOL. Letting norm() denote the Euclidean norm, ERR is defined as follows. If ITOL=0, then ERR = norm(SB*(B-A*X(L)))/norm(SB*B), for right or no preconditioning, and norm(SB*(M-inverse)*(B-A*X(L)))/ norm(SB*(M-inverse)*B), for left preconditioning. If ITOL=1, then ERR = norm(SB*(B-A*X(L)))/norm(SB*B), since right or no preconditioning being used. If ITOL=2, then ERR = norm(SB*(M-inverse)*(B-A*X(L)))/ since left preconditioning is being If ITOL=3, then ERR = Max |(Minv*(B-A*X(L)))(i)/x(i)| i=1,n If ITOL=11, then ERR = norm(SB*(X(L)-SOLN))/norm(SB*SOLN).

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Return error flag. 0 => All went well. 1 => Insufficient storage allocated for RGWK or IGWK. 2 => Routine SGMRES failed to reduce the norm of the current residual on its last call, and so the iteration has stalled. In this case, X equals the last computed approximation.

## `IUNIT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence. If unit number is 0, no writing will occur.

## `SB`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

SB(N). Array of length N containing scale factors for the right hand side vector B. If JSCAL. eq. 0 (see below), SB need not be supplied.

## `SX`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (N).

SX(N). Array of length N containing scale factors for the solution vector X. If JSCAL. eq. 0 (see below), SX need not be supplied. SB and SX can be the same array in the calling program if desired.

## `RGWK`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (LRGW).

RGWK(LRGW). array used for workspace by SGMRES. On return, RGWK(1) = RHOL. See IERR for definition of RHOL.

## `LRGW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Length of the real workspace, RGWK. 1 + N*(MAXL+6) + MAXL*(MAXL+3). See below for definition of MAXL. For the default values, RGWK has size at least 131 + 16*N.

## `IGWK`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (LIGW).

contains the required minimum length of the RGWK array. IGWK(LIGW). The following IGWK parameters should be set by the user before calling this routine. MAXL. Maximum dimension of Krylov subspace in which X - X0 is to be found (where, X0 is the initial guess). The default value of MAXL is 10.

## `LIGW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Integer. Length of the integer workspace, IGWK. LIGW >= 20.

## `RWORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

RWORK(USER DEFINED). array that can be used for workspace in MSOLVE.

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
- `SB`: not a workspace argument
- `SX`: not a workspace argument
- `RGWK`: RGWK(LRGW). array used for workspace by SGMRES. On return, RGWK(1) = RHOL. See IERR for definition of RHOL.
- `IGWK`: not a workspace argument
- `RWORK`: RWORK(USER DEFINED). array that can be used for workspace in MSOLVE.
- `IWORK`: IWORK(USER DEFINED). array that can be used for workspace in MSOLVE.

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::sgmres`
- Original SLATEC routine: `SGMRES`
- Native symbol: `sgmres_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32),mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [SGMRES](https://www.netlib.org/slatec/lin/sgmres.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
