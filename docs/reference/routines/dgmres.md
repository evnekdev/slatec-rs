# DGMRES

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Preconditioned GMRES iterative sparse Ax=b solver. This routine uses the generalized minimum residual (GMRES) method with preconditioning to solve non-symmetric linear systems of the form: Ax = b.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, LRGW, IGWK(LIGW), LIGW INTEGER IWORK(USER DEFINED) DOUBLE PRECISION B(N), X(N), A(NELT), TOL, ERR, SB(N), SX(N) DOUBLE PRECISION RGWK(LRGW), RWORK(USER DEFINED) EXTERNAL MATVEC, MSOLVE CALL DGMRES(N, B, X, NELT, IA, JA, A, ISYM, MATVEC, MSOLVE, $ ITOL, TOL, ITMAX, ITER, ERR, IERR, IUNIT, SB, SX, $ RGWK, LRGW, IGWK, LIGW, RWORK, IWORK) DGMRES solves a linear system A*X = B rewritten in the form: (SB*A*(M-inverse)*(SX-inverse))*(SX*M*X) = SB*B, with right preconditioning, or (SB*(M-inverse)*A*(SX-inverse))*(SX*X) = SB*(M-inverse)*B, with left preconditioning, where A is an N-by-N double precision matrix, X and B are N-vectors, SB and SX are diagonal scaling matrices, and M is a preconditioning matrix. It uses preconditioned Krylov subpace methods based on the generalized minimum residual method (GMRES). This routine optionally performs either the full orthogonalization version of the GMRES algorithm or an incomplete variant of it. Both versions use restarting of the linear iteration by default, although the user can disable this feature. The GMRES algorithm generates a sequence of approximations X(L) to the true solution of the above linear system. The convergence criteria for stopping the iteration is based on the size of the scaled norm of the residual R(L) = B - A*X(L). The actual stopping test is either: norm(SB*(B-A*X(L))) .le. TOL*norm(SB*B), for right preconditioning, or norm(SB*(M-inverse)*(B-A*X(L))) .le. TOL*norm(SB*(M-inverse)*B), for left preconditioning, where norm() denotes the Euclidean norm, and TOL is a positive scalar less than one input by the user. If TOL equals zero when DGMRES is called, then a default value of 500*(the smallest positive magnitude, machine epsilon) is used. If the scaling arrays SB and SX are used, then ideally they should be chosen so that the vectors SX*X(or SX*M*X) and SB*B have all their components approximately equal to one in magnitude. If one wants to use the same scaling in X and B, then SB and SX can be the same array in the calling program. The following is a list of the other routines and their functions used by DGMRES: DPIGMR Contains the main iteration loop for GMRES. DORTH Orthogonalizes a new vector against older basis vectors. DHEQR Computes a QR decomposition of a Hessenberg matrix. DHELS Solves a Hessenberg least-squares system, using QR factors. DRLCAL Computes the scaled residual RL. DXLCAL Computes the solution XL. ISDGMR User-replaceable stopping routine. This routine does not care what matrix data structure is used for A and M. It simply calls the MATVEC and MSOLVE routines, with the arguments as described above. The user could write any type of structure and the appropriate MATVEC and MSOLVE routines. It is assumed that A is stored in the IA, JA, A arrays in some fashion and that M (or INV(M)) is stored in IWORK and RWORK in some fashion. The SLAP routines DSDCG and DSICCG are examples of this procedure. Two examples of matrix data structures are the: 1) SLAP Triad format and 2) SLAP Column format. =================== S L A P Triad format =================== This routine requires that the matrix A be stored in the SLAP Triad format. In this format only the non-zeros are stored. They may appear in *ANY* order. The user supplies three arrays of length NELT, where NELT is the number of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)). For each non-zero the user puts the row and column index of that matrix element in the IA and JA arrays. The value of the non-zero matrix element is placed in the corresponding location of the A array. This is an extremely easy data structure to generate. On the other hand it is not too efficient on vector computers for the iterative solution of linear systems. Hence, SLAP changes this input data structure to the SLAP Column format for the iteration (but does not change it back). Here is an example of the SLAP Triad storage format for a 5x5 Matrix. Recall that the entries may appear in any order. 5x5 Matrix SLAP Triad format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 51 12 11 33 15 53 55 22 35 44 21 |21 22 0 0 0| IA: 5 1 1 3 1 5 5 2 3 4 2 | 0 0 33 0 35| JA: 1 2 1 3 5 3 5 2 5 4 1 | 0 0 0 44 0| |51 0 53 0 55| =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the double precision array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the end of the ICOL-th column. Note that we always have JA(N+1) = NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 |21 22 0 0 0| IA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| JA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| Cautions: This routine will attempt to write to the Fortran logical output unit IUNIT, if IUNIT .ne. 0. Thus, the user must make sure that this logical unit is attached to a file or terminal before calling this routine with a non-zero value for IUNIT. This routine does not check for the validity of a non-zero IUNIT unit number.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
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

- Canonical provider: `lin/dgmres.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dgmres.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dgmres.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Enriched from the 20-routine pilot; this catalogue is the canonical corpus view.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DGMRES](https://www.netlib.org/slatec/lin/dgmres.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | hand side vector and Z is the solution upon return.  NELT, IA, JA, A and ISYM are defined as above.  RWORK is a double precision array that can be used to pass necessary preconditioning information and/or workspace to MSOLVE.  IWORK is an integer work array for the same purpose as RWORK. ITOL   :IN       Integer. Flag to indicate the type of convergence criterion used. |
| 2 | `B` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `X` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `A` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (NELT) | zero, denotes that A is symmetric and only the lower or upper triangle is stored. MSOLVE :EXT      External. r for z given r with the preconditioning matrix M (M is supplied via RWORK and IWORK arrays.  The name of the MSOLVE routine must be declared external in the calling program.  The calling sequence to MSOLVE is: CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. MATVEC :EXT      External. Name of a routine which performs the matrix vector multiply Y = A*X given A and X.  The name of the MATVEC routine must be declared external in the calling program.  The calling sequence to MATVEC is: CALL MATVEC(N, X, Y, NELT, IA, JA, A, ISYM) where N is the number of unknowns, Y is the product A*X upon return, X is an input vector, and NELT is the number of non-zeros in the SLAP IA, JA, A storage for the matrix A. zero, denotes that A is symmetric and only the lower or upper triangle is stored. MSOLVE :EXT      External. |
| 9 | `MATVEC` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 10 | `MSOLVE` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 11 | `ITOL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 0  Means the  iteration stops when the test described below on  the  residual RL  is satisfied.  This is the  "Natural Stopping Criteria" for this routine. Other values  of   ITOL  cause  extra,   otherwise unnecessary, computation per iteration and     are therefore  much less  efficient.  See  ISDGMR (the stop test routine) for more information. 1  Means   the  iteration stops   when the first test described below on  the residual RL  is satisfied, and there  is either right  or  no preconditioning being used. 2  Implies     that   the  user    is   using    left preconditioning, and the second stopping criterion below is used. 3  Means the  iteration stops   when  the  third test described below on Minv*Residual is satisfied, and there is either left  or no  preconditioning being used. 11 is    often  useful  for   checking  and comparing different routines.  For this case, the  user must supply  the  "exact" solution or  a  very accurate |
| 12 | `TOL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 13 | `ITMAX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 14 | `ITER` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 15 | `ERR` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 16 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 17 | `IUNIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 18 | `SB` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 19 | `SX` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 20 | `RGWK` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (LRGW) | Array argument classified by fixed-form executable read/write analysis. |
| 21 | `LRGW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 22 | `IGWK` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (LIGW) | Array argument classified by fixed-form executable read/write analysis. |
| 23 | `LIGW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 24 | `RWORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |
| 25 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

TOL) through a common block, COMMON /DSLBLK/ SOLN( ) If ITOL=11, iteration stops when the 2-norm of the difference between the iterative approximation and the user-supplied solution  divided by the  2-norm of the  user-supplied solution  is  less than TOL. Note that this requires  the  user to  set up  the "COMMON     /DSLBLK/ SOLN(LENGTH)"  in the calling routine.  The routine with this declaration should be loaded before the stop test so that the correct length is used by  the loader.  This procedure  is not standard Fortran and may not work correctly on your   system (although  it  has  worked  on every system the authors have tried).  If ITOL is not 11 then this common block is indeed standard Fortran. TOL    :INOUT    Double Precision. Convergence criterion, as described below.  If TOL is set to zero on input, then a default value of 500*(the smallest positive magnitude, machine epsilon) is used. ITMAX  :DUMMY    Integer. Maximum number of iterations in most SLAP routines.  In this routine this does not make sense.  The maximum number of iterations here is given by ITMAX = MAXL*(NRMAX+1). See IGWK for definitions of MAXL and NRMAX. ITER   :OUT      Integer. Number of iterations required to reach convergence, or ITMAX if convergence criterion could not be achieved in ITMAX iterations. ERR    :OUT      Double Precision. defined by ITOL.  Letting norm() denote the Euclidean norm, ERR is defined as follows.. If ITOL=0, then ERR = norm(SB*(B-A*X(L)))/norm(SB*B), for right or no preconditioning, and ERR = norm(SB*(M-inverse)*(B-A*X(L)))/ norm(SB*(M-inverse)*B), for left preconditioning. If ITOL=1, then ERR = norm(SB*(B-A*X(L)))/norm(SB*B), since right or no preconditioning being used. If ITOL=2, then ERR = norm(SB*(M-inverse)*(B-A*X(L)))/ norm(SB*(M-inverse)*B), since left preconditioning is being used. If ITOL=3, then ERR =  Max  |(Minv*(B-A*X(L)))(i)/x(i)| i=1,n If ITOL=11, then ERR = norm(SB*(X(L)-SOLN))/norm(SB*SOLN). IERR   :OUT      Integer. IERR = 0 => All went well. IERR = 1 => Insufficient storage allocated for RGWK or IGWK. IERR = 2 => Routine DGMRES failed to reduce the norm of the current residual on its last call, and so the iteration has stalled.  In this case, X equals the last computed approximation.  The user must either increase MAXL, or choose a different initial guess. IERR =-1 => Insufficient length for RGWK array. IGWK(6) contains the required minimum length of the RGWK array. IERR =-2 => Illegal value of ITOL, or ITOL and JPRE values are inconsistent. For IERR <= 2, RGWK(1) = RHOL, which is the norm on the left-hand-side of the relevant stopping test defined below associated with the residual for the current approximation X(L). IUNIT  :IN       Integer. if this is desired for monitoring convergence.  If unit number is 0, no writing will occur. SB     :IN       Double Precision SB(N). Array of length N containing scale factors for the right hand side vector B.  If JSCAL.eq.0 (see below), SB need not be supplied. SX     :IN       Double Precision SX(N). Array of length N containing scale factors for the solution vector X.  If JSCAL.eq.0 (see below), SX need not be supplied.  SB and SX can be the same array in the calling program if desired. RGWK   :INOUT    Double Precision RGWK(LRGW). Double Precision array used for workspace by DGMRES. On return, RGWK(1) = RHOL.  See IERR for definition of RHOL. LRGW   :IN       Integer. Length of the double precision workspace, RGWK. LRGW >= 1 + N*(MAXL+6) + MAXL*(MAXL+3). See below for definition of MAXL. For the default values, RGWK has size at least 131 + 16*N. IGWK   :INOUT    Integer IGWK(LIGW). The following IGWK parameters should be set by the user before calling this routine. IGWK(1) = MAXL.  Maximum dimension of Krylov subspace in which X - X0 is to be found (where, X0 is the initial guess).  The default value of MAXL is 10. IGWK(2) = KMP.  Maximum number of previous Krylov basis vectors to which each new basis vector is made orthogonal. The default value of KMP is MAXL. IGWK(3) = JSCAL.  Flag indicating whether the scaling arrays SB and SX are to be used. JSCAL = 0 => SB and SX are not used and the algorithm will perform as if all SB(I) = 1 and SX(I) = 1. JSCAL = 1 =>  Only SX is used, and the algorithm performs as if all SB(I) = 1. JSCAL = 2 =>  Only SB is used, and the algorithm performs as if all SX(I) = 1. JSCAL = 3 =>  Both SB and SX are used. IGWK(4) = JPRE.  Flag indicating whether preconditioning is being used. JPRE = 0  =>  There is no preconditioning. JPRE > 0  =>  There is preconditioning on the right only, and the solver will call routine MSOLVE. JPRE < 0  =>  There is preconditioning on the left only, and the solver will call routine MSOLVE. IGWK(5) = NRMAX.  Maximum number of restarts of the Krylov iteration.  The default value of NRMAX = 10. if IWORK(5) = -1,  then no restarts are performed (in this case, NRMAX is set to zero internally). The following IWORK parameters are diagnostic information made available to the user after this routine completes. IGWK(6) = MLWK.  Required minimum length of RGWK array. IGWK(7) = NMS.  The total number of calls to MSOLVE. LIGW   :IN       Integer. Length of the integer workspace, IGWK.  LIGW >= 20. RWORK  :WORK     Double Precision RWORK(USER DEFINED). Double Precision array that can be used for workspace in MSOLVE. IWORK  :WORK     Integer IWORK(USER DEFINED). Integer array that can be used for workspace in MSOLVE.

### Storage and workspace requirements

`RWORK`: Workspace argument classified by fixed-form executable read/write analysis.

`IWORK`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::dgmres`. Native symbol: `dgmres_`. Declaration feature: `linear-algebra-iterative`. Provider feature: `linear-algebra`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::dgmres`
- Public declaration feature: `linear-algebra-iterative`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
