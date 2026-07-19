# SSLUGM

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Incomplete LU GMRES Iterative Sparse Ax=b Solver. This routine uses the generalized minimum residual (GMRES) method with incomplete LU factorization for preconditioning to solve possibly non-symmetric linear systems of the form: Ax = b.

## Description

*Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, NSAVE, ITOL INTEGER ITMAX, ITER, IERR, IUNIT, LENW, IWORK(LENIW), LENIW REAL B(N), X(N), A(NELT), TOL, ERR, RWORK(LENW) CALL SSLUGM(N, B, X, NELT, IA, JA, A, ISYM, NSAVE, $ ITOL, TOL, ITMAX, ITER, ERR, IERR, IUNIT, $ RWORK, LENW, IWORK, LENIW) *Arguments: N :IN Integer. Order of the Matrix. B :IN Real B(N). Right-hand side vector. X :INOUT Real X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution. NELT :IN Integer. Number of Non-Zeros stored in A. IA :IN Integer IA(NELT). JA :IN Integer JA(NELT). A :IN Real A(NELT). These arrays should hold the matrix A in either the SLAP Triad format or the SLAP Column format. See "Description", below. If the SLAP Triad format is chosen it is changed internally to the SLAP Column format. ISYM :IN Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. NSAVE :IN Integer. Number of direction vectors to save and orthogonalize against. Must be greater than 1. ITOL :IN Integer. Flag to indicate the type of convergence criterion used. ITOL=0 Means the iteration stops when the test described below on the residual RL is satisfied. This is the "Natural Stopping Criteria" for this routine. Other values of ITOL cause extra, otherwise unnecessary, computation per iteration and are therefore much less efficient. See ISSGMR (the stop test routine) for more information. ITOL=1 Means the iteration stops when the first test described below on the residual RL is satisfied, and there is either right or no preconditioning being used. ITOL=2 Implies that the user is using left preconditioning, and the second stopping criterion below is used. ITOL=3 Means the iteration stops when the third test described below on Minv*Residual is satisfied, and there is either left or no preconditioning begin used. ITOL=11 is often useful for checking and comparing different routines. For this case, the user must supply the "exact" solution or a very accurate approximation (one with an error much less than TOL) through a common block, COMMON /SSLBLK/ SOLN( ) If ITOL=11, iteration stops when the 2-norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL. Note that this requires the user to set up the "COMMON /SSLBLK/ SOLN(LENGTH)" in the calling routine. The routine with this declaration should be loaded before the stop test so that the correct length is used by the loader. This procedure is not standard Fortran and may not work correctly on your system (although it has worked on every system the authors have tried). If ITOL is not 11 then this common block is indeed standard Fortran. TOL :INOUT Real. Convergence criterion, as described below. If TOL is set to zero on input, then a default value of 500*(the smallest positive magnitude, machine epsilon) is used. ITMAX :IN Integer. Maximum number of iterations. This routine uses the default of NRMAX = ITMAX/NSAVE to determine the when each restart should occur. See the description of NRMAX and MAXL in SGMRES for a full and frightfully interesting discussion of this topic. ITER :OUT Integer. Number of iterations required to reach convergence, or ITMAX+1 if convergence criterion could not be achieved in ITMAX iterations. ERR :OUT Real. Error estimate of error in final approximate solution, as defined by ITOL. Letting norm() denote the Euclidean norm, ERR is defined as follows... If ITOL=0, then ERR = norm(SB*(B-A*X(L)))/norm(SB*B), for right or no preconditioning, and ERR = norm(SB*(M-inverse)*(B-A*X(L)))/ norm(SB*(M-inverse)*B), for left preconditioning. If ITOL=1, then ERR = norm(SB*(B-A*X(L)))/norm(SB*B), since right or no preconditioning being used. If ITOL=2, then ERR = norm(SB*(M-inverse)*(B-A*X(L)))/ norm(SB*(M-inverse)*B), since left preconditioning is being used. If ITOL=3, then ERR = Max |(Minv*(B-A*X(L)))(i)/x(i)| i=1,n If ITOL=11, then ERR = norm(SB*(X(L)-SOLN))/norm(SB*SOLN). IERR :OUT Integer. Return error flag. IERR = 0 => All went well. IERR = 1 => Insufficient storage allocated for RGWK or IGWK. IERR = 2 => Routine SPIGMR failed to reduce the norm of the current residual on its last call, and so the iteration has stalled. In this case, X equals the last computed approximation. The user must either increase MAXL, or choose a different initial guess. IERR =-1 => Insufficient length for RGWK array. IGWK(6) contains the required minimum length of the RGWK array. IERR =-2 => Inconsistent ITOL and JPRE values. For IERR <= 2, RGWK(1) = RHOL, which is the norm on the left-hand-side of the relevant stopping test defined below associated with the residual for the current approximation X(L). IUNIT :IN Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence. If unit number is 0, no writing will occur. RWORK :WORK Real RWORK(LENW). Real array of size LENW. LENW :IN Integer. Length of the real workspace, RWORK. LENW >= 1 + N*(NSAVE+7) + NSAVE*(NSAVE+3)+NL+NU. Here NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal) and NU is the number of non-zeros in the upper triangle of the matrix (including the diagonal). For the recommended values, RWORK has size at least 131 + 17*N + NL + NU. IWORK :INOUT Integer IWORK(LENIW). Used to hold pointers into the RWORK array. Upon return the following locations of IWORK hold information which may be of use to the user: IWORK(9) Amount of Integer workspace actually used. IWORK(10) Amount of Real workspace actually used. LENIW :IN Integer. Length of the integer workspace, IWORK. LENIW >= NL+NU+4*N+32. *Description: SSLUGM solves a linear system A*X = B rewritten in the form: (SB*A*(M-inverse)*(SX-inverse))*(SX*M*X) = SB*B, with right preconditioning, or (SB*(M-inverse)*A*(SX-inverse))*(SX*X) = SB*(M-inverse)*B, with left preconditioning, where A is an n-by-n real matrix, X and B are N-vectors, SB and SX are diagonal scaling matrices, and M is the Incomplete LU factorization of A. It uses preconditioned Krylov subpace methods based on the generalized minimum residual method (GMRES). This routine is a driver routine which assumes a SLAP matrix data structure and sets up the necessary information to do diagonal preconditioning and calls the main GMRES routine SGMRES for the solution of the linear system. SGMRES optionally performs either the full orthogonalization version of the GMRES algorithm or an incomplete variant of it. Both versions use restarting of the linear iteration by default, although the user can disable this feature. The GMRES algorithm generates a sequence of approximations X(L) to the true solution of the above linear system. The convergence criteria for stopping the iteration is based on the size of the scaled norm of the residual R(L) = B A*X(L). The actual stopping test is either: norm(SB*(B-A*X(L))) .le. TOL*norm(SB*B), for right preconditioning, or norm(SB*(M-inverse)*(B-A*X(L))) .le. TOL*norm(SB*(M-inverse)*B), for left preconditioning, where norm() denotes the Euclidean norm, and TOL is a positive scalar less than one input by the user. If TOL equals zero when SSLUGM is called, then a default value of 500*(the smallest positive magnitude, machine epsilon) is used. If the scaling arrays SB and SX are used, then ideally they should be chosen so that the vectors SX*X(or SX*M*X) and SB*B have all their components approximately equal to one in magnitude. If one wants to use the same scaling in X and B, then SB and SX can be the same array in the calling program. The following is a list of the other routines and their functions used by GMRES: SGMRES Contains the matrix structure independent driver routine for GMRES. SPIGMR Contains the main iteration loop for GMRES. SORTH Orthogonalizes a new vector against older basis vectors. SHEQR Computes a QR decomposition of a Hessenberg matrix. SHELS Solves a Hessenberg least-squares system, using QR factors. RLCALC Computes the scaled residual RL. XLCALC Computes the solution XL. ISSGMR User-replaceable stopping routine. The Sparse Linear Algebra Package (SLAP) utilizes two matrix data structures: 1) the SLAP Triad format or 2) the SLAP Column format. The user can hand this routine either of the of these data structures and SLAP will figure out which on is being used and act accordingly. =================== S L A P Triad format =================== This routine requires that the matrix A be stored in the SLAP Triad format. In this format only the non-zeros are stored. They may appear in *ANY* order. The user supplies three arrays of length NELT, where NELT is the number of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)). For each non-zero the user puts the row and column index of that matrix element in the IA and JA arrays. The value of the non-zero matrix element is placed in the corresponding location of the A array. This is an extremely easy data structure to generate. On the other hand it is not too efficient on vector computers for the iterative solution of linear systems. Hence, SLAP changes this input data structure to the SLAP Column format for the iteration (but does not change it back). Here is an example of the SLAP Triad storage format for a 5x5 Matrix. Recall that the entries may appear in any order. 5x5 Matrix SLAP Triad format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 51 12 11 33 15 53 55 22 35 44 21 |21 22 0 0 0| IA: 5 1 1 3 1 5 5 2 3 4 2 | 0 0 33 0 35| JA: 1 2 1 3 5 3 5 2 5 4 1 | 0 0 0 44 0| |51 0 53 0 55| =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the

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

- Canonical provider: `lin/sslugm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sslugm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sslugm.f) — `verified_cached`
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
