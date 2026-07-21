# SSLUGM

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Incomplete LU GMRES Iterative Sparse Ax=b Solver. This routine uses the generalized minimum residual (GMRES) method with incomplete LU factorization for preconditioning to solve possibly non-symmetric linear systems of the form: Ax = b.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, NSAVE, ITOL INTEGER ITMAX, ITER, IERR, IUNIT, LENW, IWORK(LENIW), LENIW REAL B(N), X(N), A(NELT), TOL, ERR, RWORK(LENW) CALL SSLUGM(N, B, X, NELT, IA, JA, A, ISYM, NSAVE, $ ITOL, TOL, ITMAX, ITER, ERR, IERR, IUNIT, $ RWORK, LENW, IWORK, LENIW) SSLUGM solves a linear system A*X = B rewritten in the form: (SB*A*(M-inverse)*(SX-inverse))*(SX*M*X) = SB*B, with right preconditioning, or (SB*(M-inverse)*A*(SX-inverse))*(SX*X) = SB*(M-inverse)*B, with left preconditioning, where A is an n-by-n real matrix, X and B are N-vectors, SB and SX are diagonal scaling matrices, and M is the Incomplete LU factorization of A. It uses preconditioned Krylov subpace methods based on the generalized minimum residual method (GMRES). This routine is a driver routine which assumes a SLAP matrix data structure and sets up the necessary information to do diagonal preconditioning and calls the main GMRES routine SGMRES for the solution of the linear system. SGMRES optionally performs either the full orthogonalization version of the GMRES algorithm or an incomplete variant of it. Both versions use restarting of the linear iteration by default, although the user can disable this feature. The GMRES algorithm generates a sequence of approximations X(L) to the true solution of the above linear system. The convergence criteria for stopping the iteration is based on the size of the scaled norm of the residual R(L) = B - A*X(L). The actual stopping test is either: norm(SB*(B-A*X(L))) .le. TOL*norm(SB*B), for right preconditioning, or norm(SB*(M-inverse)*(B-A*X(L))) .le. TOL*norm(SB*(M-inverse)*B), for left preconditioning, where norm() denotes the Euclidean norm, and TOL is a positive scalar less than one input by the user. If TOL equals zero when SSLUGM is called, then a default value of 500*(the smallest positive magnitude, machine epsilon) is used. If the scaling arrays SB and SX are used, then ideally they should be chosen so that the vectors SX*X(or SX*M*X) and SB*B have all their components approximately equal to one in magnitude. If one wants to use the same scaling in X and B, then SB and SX can be the same array in the calling program. The following is a list of the other routines and their functions used by GMRES: SGMRES Contains the matrix structure independent driver routine for GMRES. SPIGMR Contains the main iteration loop for GMRES. SORTH Orthogonalizes a new vector against older basis vectors. SHEQR Computes a QR decomposition of a Hessenberg matrix. SHELS Solves a Hessenberg least-squares system, using QR factors. RLCALC Computes the scaled residual RL. XLCALC Computes the solution XL. ISSGMR User-replaceable stopping routine. The Sparse Linear Algebra Package (SLAP) utilizes two matrix data structures: 1) the SLAP Triad format or 2) the SLAP Column format. The user can hand this routine either of the of these data structures and SLAP will figure out which on is being used and act accordingly. =================== S L A P Triad format =================== This routine requires that the matrix A be stored in the SLAP Triad format. In this format only the non-zeros are stored. They may appear in *ANY* order. The user supplies three arrays of length NELT, where NELT is the number of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)). For each non-zero the user puts the row and column index of that matrix element in the IA and JA arrays. The value of the non-zero matrix element is placed in the corresponding location of the A array. This is an extremely easy data structure to generate. On the other hand it is not too efficient on vector computers for the iterative solution of linear systems. Hence, SLAP changes this input data structure to the SLAP Column format for the iteration (but does not change it back). Here is an example of the SLAP Triad storage format for a 5x5 Matrix. Recall that the entries may appear in any order. 5x5 Matrix SLAP Triad format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 51 12 11 33 15 53 55 22 35 44 21 |21 22 0 0 0| IA: 5 1 1 3 1 5 5 2 3 4 2 | 0 0 33 0 35| JA: 1 2 1 3 5 3 5 2 5 4 1 | 0 0 0 44 0| |51 0 53 0 55| =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the end of the ICOL-th column. Note that we always have JA(N+1) = NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 |21 22 0 0 0| IA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| JA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| Side Effects: The SLAP Triad format (IA, JA, A) is modified internally to be the SLAP Column format. See above. Cautions: This routine will attempt to write to the Fortran logical output unit IUNIT, if IUNIT .ne. 0. Thus, the user must make sure that this logical unit is attached to a file or terminal before calling this routine with a non-zero value for IUNIT. This routine does not check for the validity of a non-zero IUNIT unit number.

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

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [SSLUGM](https://www.netlib.org/slatec/lin/sslugm.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NELT) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. NSAVE  :IN       Integer. Number of direction vectors to save and orthogonalize against. Must be greater than 1. ITOL   :IN       Integer. Flag to indicate the type of convergence criterion used. |
| 9 | `NSAVE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 10 | `ITOL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 0  Means the  iteration stops when the test described below on  the  residual RL  is satisfied.  This is the  "Natural Stopping Criteria" for this routine. Other values  of   ITOL  cause  extra,   otherwise unnecessary, computation per iteration and     are therefore  much less  efficient.  See  ISSGMR (the stop test routine) for more information. 1  Means   the  iteration stops   when the first test described below on  the residual RL  is satisfied, and there  is either right  or  no preconditioning being used. 2  Implies     that   the  user    is   using    left preconditioning, and the second stopping criterion below is used. 3  Means the  iteration stops   when  the  third test described below on Minv*Residual is satisfied, and there is either left  or no  preconditioning begin used. 11 is    often  useful  for   checking  and comparing different routines.  For this case, the  user must supply  the  "exact" solution or  a  very accurate |
| 11 | `TOL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 12 | `ITMAX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 13 | `ITER` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 14 | `ERR` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 15 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 16 | `IUNIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 17 | `RWORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (LENW) | Workspace argument classified by fixed-form executable read/write analysis. |
| 18 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 19 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (LENIW) | Workspace argument classified by fixed-form executable read/write analysis. |
| 20 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

TOL) through a common block, COMMON /SSLBLK/ SOLN( ) If ITOL=11, iteration stops when the 2-norm of the difference between the iterative approximation and the user-supplied solution  divided by the  2-norm of the  user-supplied solution  is  less than TOL. Note that this requires  the  user to  set up  the "COMMON     /SSLBLK/ SOLN(LENGTH)"  in the calling routine.  The routine with this declaration should be loaded before the stop test so that the correct length is used by  the loader.  This procedure  is not standard Fortran and may not work correctly on your   system (although  it  has  worked  on every system the authors have tried).  If ITOL is not 11 then this common block is indeed standard Fortran. TOL    :INOUT    Real. Convergence criterion, as described below.  If TOL is set to zero on input, then a default value of 500*(the smallest positive magnitude, machine epsilon) is used. ITMAX  :IN       Integer. Maximum number of iterations.  This routine uses the default of NRMAX = ITMAX/NSAVE to determine the when each restart should occur.  See the description of NRMAX and MAXL in SGMRES for a full and frightfully interesting discussion of this topic. ITER   :OUT      Integer. Number of iterations required to reach convergence, or ITMAX+1 if convergence criterion could not be achieved in ITMAX iterations. ERR    :OUT      Real. defined by ITOL.  Letting norm() denote the Euclidean norm, ERR is defined as follows... If ITOL=0, then ERR = norm(SB*(B-A*X(L)))/norm(SB*B), for right or no preconditioning, and ERR = norm(SB*(M-inverse)*(B-A*X(L)))/ norm(SB*(M-inverse)*B), for left preconditioning. If ITOL=1, then ERR = norm(SB*(B-A*X(L)))/norm(SB*B), since right or no preconditioning being used. If ITOL=2, then ERR = norm(SB*(M-inverse)*(B-A*X(L)))/ norm(SB*(M-inverse)*B), since left preconditioning is being used. If ITOL=3, then ERR =  Max  |(Minv*(B-A*X(L)))(i)/x(i)| i=1,n If ITOL=11, then ERR = norm(SB*(X(L)-SOLN))/norm(SB*SOLN). IERR   :OUT      Integer. IERR = 0 => All went well. IERR = 1 => Insufficient storage allocated for RGWK or IGWK. IERR = 2 => Routine SPIGMR failed to reduce the norm of the current residual on its last call, and so the iteration has stalled.  In this case, X equals the last computed approximation.  The user must either increase MAXL, or choose a different initial guess. IERR =-1 => Insufficient length for RGWK array. IGWK(6) contains the required minimum length of the RGWK array. IERR =-2 => Inconsistent ITOL and JPRE values. For IERR <= 2, RGWK(1) = RHOL, which is the norm on the left-hand-side of the relevant stopping test defined below associated with the residual for the current approximation X(L). IUNIT  :IN       Integer. if this is desired for monitoring convergence.  If unit number is 0, no writing will occur. RWORK  :WORK    Real RWORK(LENW). Real array of size LENW. LENW   :IN       Integer. Length of the real workspace, RWORK. LENW >= 1 + N*(NSAVE+7) +  NSAVE*(NSAVE+3)+NL+NU. Here NL is the number of non-zeros in the lower triangle of the matrix (including the diagonal) and NU is the number of non-zeros in the upper triangle of the matrix (including the diagonal). For the recommended values,  RWORK  has size at least 131 + 17*N + NL + NU. IWORK  :INOUT    Integer IWORK(LENIW). Used to hold pointers into the RWORK array. Upon return the following locations of IWORK hold information which may be of use to the user: IWORK(9)  Amount of Integer workspace actually used. IWORK(10) Amount of Real workspace actually used. LENIW  :IN       Integer. Length of the integer workspace, IWORK. LENIW >= NL+NU+4*N+32.

### Storage and workspace requirements

`RWORK`: Workspace argument classified by fixed-form executable read/write analysis.

`IWORK`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::sparse::sslugm`. Native symbol: `sslugm_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::sparse::sslugm`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
