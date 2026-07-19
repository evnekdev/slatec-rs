# SIR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Preconditioned Iterative Refinement Sparse Ax = b Solver. Routine to solve a general linear system Ax = b using iterative refinement with a matrix splitting.

## Description

*Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, IWORK(USER DEFINED) REAL B(N), X(N), A(NELT), TOL, ERR, R(N), Z(N), DZ(N), REAL RWORK(USER DEFINED) EXTERNAL MATVEC, MSOLVE CALL SIR(N, B, X, NELT, IA, JA, A, ISYM, MATVEC, MSOLVE, ITOL, $ TOL, ITMAX, ITER, ERR, IERR, IUNIT, R, Z, DZ, RWORK, IWORK) *Arguments: N :IN Integer. Order of the Matrix. B :IN Real B(N). Right-hand side vector. X :INOUT Real X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution. NELT :IN Integer. Number of Non-Zeros stored in A. IA :IN Integer IA(NELT). JA :IN Integer JA(NELT). A :IN Real A(NELT). These arrays contain the matrix data structure for A. It could take any form. See "Description", below, for more details. ISYM :IN Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. MATVEC :EXT External. Name of a routine which performs the matrix vector multiply Y = A*X given A and X. The name of the MATVEC routine must be declared external in the calling program. The calling sequence to MATVEC is: CALL MATVEC( N, X, Y, NELT, IA, JA, A, ISYM ) Where N is the number of unknowns, Y is the product A*X upon return, X is an input vector, NELT is the number of non-zeros in the SLAP IA, JA, A storage for the matrix A. ISYM is a flag which, if non-zero, denotes that A is symmetric and only the lower or upper triangle is stored. MSOLVE :EXT External. Name of a routine which solves a linear system MZ = R for Z given R with the preconditioning matrix M (M is supplied via RWORK and IWORK arrays). The name of the MSOLVE routine must be declared external in the calling program. The calling sequence to MSOLVE is: CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) Where N is the number of unknowns, R is the right-hand side vector and Z is the solution upon return. NELT, IA, JA, A and ISYM are defined as above. RWORK is a real array that can be used to pass necessary preconditioning information and/or workspace to MSOLVE. IWORK is an integer work array for the same purpose as RWORK. ITOL :IN Integer. Flag to indicate type of convergence criterion. If ITOL=1, iteration stops when the 2-norm of the residual divided by the 2-norm of the right-hand side is less than TOL. If ITOL=2, iteration stops when the 2-norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand side is less than TOL, where M-inv is the inverse of the diagonal of A. ITOL=11 is often useful for checking and comparing different routines. For this case, the user must supply the "exact" solution or a very accurate approximation (one with an error much less than TOL) through a common block, COMMON /SSLBLK/ SOLN( ) If ITOL=11, iteration stops when the 2-norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL. Note that this requires the user to set up the "COMMON /SSLBLK/ SOLN(LENGTH)" in the calling routine. The routine with this declaration should be loaded before the stop test so that the correct length is used by the loader. This procedure is not standard Fortran and may not work correctly on your system (although it has worked on every system the authors have tried). If ITOL is not 11 then this common block is indeed standard Fortran. TOL :INOUT Real. Convergence criterion, as described above. (Reset if IERR=4.) ITMAX :IN Integer. Maximum number of iterations. ITER :OUT Integer. Number of iterations required to reach convergence, or ITMAX+1 if convergence criterion could not be achieved in ITMAX iterations. ERR :OUT Real. Error estimate of error in final approximate solution, as defined by ITOL. IERR :OUT Integer. Return error flag. IERR = 0 => All went well. IERR = 1 => Insufficient space allocated for WORK or IWORK. IERR = 2 => Method failed to converge in ITMAX steps. IERR = 3 => Error in user input. Check input values of N, ITOL. IERR = 4 => User error tolerance set too tight. Reset to 500*R1MACH(3). Iteration proceeded. IERR = 5 => Preconditioning matrix, M, is not positive definite. (r,z) < 0. IERR = 6 => Matrix A is not positive definite. (p,Ap) < 0. IUNIT :IN Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence. If unit number is 0, no writing will occur. R :WORK Real R(N). Z :WORK Real Z(N). DZ :WORK Real DZ(N). Real arrays used for workspace. RWORK :WORK Real RWORK(USER DEFINED). Real array that can be used by MSOLVE. IWORK :WORK Integer IWORK(USER DEFINED). Integer array that can be used by MSOLVE. *Description: The basic algorithm for iterative refinement (also known as iterative improvement) is: n+1 n -1 n X = X + M (B - AX ). -1 -1 If M = A then this is the standard iterative refinement algorithm and the "subtraction" in the residual calculation should be done in double precision (which it is not in this routine). If M = DIAG(A), the diagonal of A, then iterative refinement is known as Jacobi's method. The SLAP routine SSJAC implements this iterative strategy. If M = L, the lower triangle of A, then iterative refinement is known as Gauss-Seidel. The SLAP routine SSGS implements this iterative strategy. This routine does not care what matrix data structure is used for A and M. It simply calls the MATVEC and MSOLVE routines, with the arguments as described above. The user could write any type of structure and the appropriate MATVEC and MSOLVE routines. It is assumed that A is stored in the IA, JA, A arrays in some fashion and that M (or INV(M)) is stored in IWORK and RWORK) in some fashion. The SLAP

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
- Raw-binding status: `not_bound`
- Build/profile status: `outside_current_immutable_snapshot`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/sir.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sir.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sir.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
