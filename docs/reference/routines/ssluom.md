# SSLUOM

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Incomplete LU Orthomin Sparse Iterative Ax=b Solver. Routine to solve a general linear system Ax = b using the Orthomin method with Incomplete LU decomposition.

## Description

*Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, NSAVE, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, LENW, IWORK(NL+NU+4*N+2), LENIW REAL B(N), X(N), A(NELT), TOL, ERR REAL RWORK(NL+NU+7*N+3*N*NSAVE+NSAVE) CALL SSLUOM(N, B, X, NELT, IA, JA, A, ISYM, NSAVE, ITOL, TOL, $ ITMAX, ITER, ERR, IERR, IUNIT, RWORK, LENW, IWORK, LENIW ) *Arguments: N :IN Integer. Order of the matrix. B :IN Real B(N). Right-hand side vector. X :INOUT Real X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution. NELT :IN Integer. Number of Non-Zeros stored in A. IA :INOUT Integer IA(NELT). JA :INOUT Integer JA(NELT). A :INOUT Real A(NELT). These arrays should hold the matrix A in either the SLAP Triad format or the SLAP Column format. See "Description", below. If the SLAP Triad format is chosen, it is changed internally to the SLAP Column format. ISYM :IN Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. NSAVE :IN Integer. Number of direction vectors to save and orthogonalize against. ITOL :IN Integer. Flag to indicate type of convergence criterion. If ITOL=1, iteration stops when the 2-norm of the residual divided by the 2-norm of the right-hand side is less than TOL. If ITOL=2, iteration stops when the 2-norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand side is less than TOL, where M-inv is the inverse of the diagonal of A. ITOL=11 is often useful for checking and comparing different routines. For this case, the user must supply the "exact" solution or a very accurate approximation (one with an error much less than TOL) through a common block, COMMON /SSLBLK/ SOLN( ) If ITOL=11, iteration stops when the 2-norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL. Note that this requires the user to set up the "COMMON /SSLBLK/ SOLN(LENGTH)" in the calling routine. The routine with this declaration should be loaded before the stop test so that the correct length is used by the loader. This procedure is not standard Fortran and may not work correctly on your system (although it has worked on every system the authors have tried). If ITOL is not 11 then this common block is indeed standard Fortran. TOL :INOUT Real. Convergence criterion, as described above. (Reset if IERR=4.) ITMAX :IN Integer. Maximum number of iterations. ITER :OUT Integer. Number of iterations required to reach convergence, or ITMAX+1 if convergence criterion could not be achieved in ITMAX iterations. ERR :OUT Real. Error estimate of error in final approximate solution, as defined by ITOL. IERR :OUT Integer. Return error flag. IERR = 0 => All went well. IERR = 1 => Insufficient space allocated for WORK or IWORK. IERR = 2 => Method failed to converge in ITMAX steps. IERR = 3 => Error in user input. Check input values of N, ITOL. IERR = 4 => User error tolerance set too tight. Reset to 500*R1MACH(3). Iteration proceeded. IERR = 5 => Preconditioning matrix, M, is not positive definite. (r,z) < 0. IERR = 6 => Breakdown of the method detected. (p,Ap) < epsilon**2. IERR = 7 => Incomplete factorization broke down and was fudged. Resulting preconditioning may be less than the best. IUNIT :IN Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence. If unit number is 0, no writing will occur. RWORK :WORK Real RWORK(LENW). Real array used for workspace. NL is the number of nonzeros in the lower triangle of the matrix (including the diagonal). NU is the number of non-zeros in the upper triangle of the matrix (including the diagonal). LENW :IN Integer. Length of the real workspace, RWORK. LENW >= NL+NU+4*N+NSAVE*(3*N+1) IWORK :WORK Integer IWORK(LENIW) Integer array used for workspace. NL is the number of nonzeros in the lower triangle of the matrix (including the diagonal). NU is the number of non-zeros in the upper triangle of the matrix (including the diagonal). Upon return the following locations of IWORK hold information which may be of use to the user: IWORK(9) Amount of Integer workspace actually used. IWORK(10) Amount of Real workspace actually used. LENIW :IN Integer. Length of the integer workspace, IWORK. LENIW >= NL+NU+4*N+12. *Description: This routine is simply a driver for the SOMN routine. It calls the SSILUS routine to set up the preconditioning and then calls SOMN with the appropriate MATVEC and MSOLVE routines. The Sparse Linear Algebra Package (SLAP) utilizes two matrix data structures: 1) the SLAP Triad format or 2) the SLAP Column format. The user can hand this routine either of the of these data structures and SLAP will figure out which on is being used and act accordingly. =================== S L A P Triad format =================== This routine requires that the matrix A be stored in the SLAP Triad format. In this format only the non-zeros are stored. They may appear in *ANY* order. The user supplies three arrays of length NELT, where NELT is the number of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)). For each non-zero the user puts the row and column index of that matrix element in the IA and JA arrays. The value of the non-zero matrix element is placed in the corresponding location of the A array. This is an extremely easy data structure to generate. On the other hand it is not too efficient on vector computers for the iterative solution of linear systems. Hence, SLAP changes this input data structure to the SLAP Column format for the iteration (but does not change it back). Here is an example of the SLAP Triad storage format for a 5x5 Matrix. Recall that the entries may appear in any order. 5x5 Matrix SLAP Triad format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 51 12 11 33 15 53 55 22 35 44 21 |21 22 0 0 0| IA: 5 1 1 3 1 5 5 2 3 4 2 | 0 0 33 0 35| JA: 1 2 1 3 5 3 5 2 5 4 1 | 0 0 0 44 0| |51 0 53 0 55| =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the

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

- Canonical provider: `lin/ssluom.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssluom.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/ssluom.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
