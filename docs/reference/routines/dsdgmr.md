# DSDGMR

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Diagonally scaled GMRES iterative sparse Ax=b solver. This routine uses the generalized minimum residual (GMRES) method with diagonal scaling to solve possibly non-symmetric linear systems of the form: Ax = b.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, NSAVE, ITOL INTEGER ITMAX, ITER, IERR, IUNIT, LENW, IWORK(LENIW), LENIW DOUBLE PRECISION B(N), X(N), A(NELT), TOL, ERR, RWORK(LENW) CALL DSDGMR(N, B, X, NELT, IA, JA, A, ISYM, NSAVE, $ ITOL, TOL, ITMAX, ITER, ERR, IERR, IUNIT, $ RWORK, LENW, IWORK, LENIW)

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
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/dsdgmr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dsdgmr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dsdgmr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DSDGMR](https://www.netlib.org/slatec/lin/dsdgmr.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Order of the Matrix. by-n double precision vectors, SB and SX are diagonal scaling matrices, and  M   is   the  diagonal  of   A.     It   uses preconditioned   Krylov  subpace   methods  based    on  the generalized  minimum residual method (GMRES).   This routine is  a  driver routine  which   assumes a  SLAP matrix   data structure  and   sets  up the  necessary information   to do diagonal preconditioning and  calls  the main GMRES  routine DGMRES   for  the  solution  of the   linear system.  DGMRES optionally   performs   either the   full  orthogonalization version of the GMRES algorithm or an  incomplete  variant of it.  Both versions use restarting of the linear iteration by default, although the user can disable this feature. The GMRES  algorithm generates a sequence  of approximations NELT+1,  where N is  the number of columns in  the matrix and NELT  is the number  of non-zeros in the matrix. Here is an example of the  SLAP Column  storage format for a |
| 2 | `B` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | IN       Double Precision B(N). Right-hand side vector. A*X(L)))/norm(SB*B), for right or no preconditioning, and for left preconditioning. A*X(L)))/norm(SB*B), since right or no preconditioning being used. since left preconditioning is being used. A*X(L)))(i)/x(i)\| vectors, SB and SX are diagonal scaling matrices, and  M   is   the  diagonal  of   A.     It   uses preconditioned   Krylov  subpace   methods  based    on  the generalized  minimum residual method (GMRES).   This routine is  a  driver routine  which   assumes a  SLAP matrix   data structure  and   sets  up the  necessary information   to do diagonal preconditioning and  calls  the main GMRES  routine DGMRES   for  the  solution  of the   linear system.  DGMRES optionally   performs   either the   full  orthogonalization version of the GMRES algorithm or an  incomplete  variant of it.  Both versions use restarting of the linear iteration by default, although the user can disable this feature. The GMRES  algorithm generates a sequence  of approximations A*X(L))) .le. TOL*norm(SB*B), for right preconditioning, or .le. |
| 3 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | INOUT    Double Precision X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution. SOLN))/norm(SB*SOLN). = SB*(M-inverse)*B, vectors, SB and SX are diagonal scaling matrices, and  M   is   the  diagonal  of   A.     It   uses preconditioned   Krylov  subpace   methods  based    on  the generalized  minimum residual method (GMRES).   This routine is  a  driver routine  which   assumes a  SLAP matrix   data structure  and   sets  up the  necessary information   to do diagonal preconditioning and  calls  the main GMRES  routine DGMRES   for  the  solution  of the   linear system.  DGMRES optionally   performs   either the   full  orthogonalization version of the GMRES algorithm or an  incomplete  variant of it.  Both versions use restarting of the linear iteration by default, although the user can disable this feature. The GMRES  algorithm generates a sequence  of approximations to the  true solution of the above  linear system.  The convergence criteria for stopping the  iteration is based on .le. |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Number of Non-Zeros stored in A. is  the number  of is  the number  of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)).  For non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)).  For each non-zero the user puts the row and column index of that each non-zero the user puts the row and column index of that matrix element  in the IA and  JA arrays.  The  value of the matrix element  in the IA and  JA arrays.  The  value of the non-zero   matrix  element is  placed  in  the corresponding non-zero   matrix  element is  placed  in  the corresponding location of the A array.   This is  an  extremely  easy data location of the A array.   This is  an  extremely  easy data structure to generate.  On  the  other hand it   is  not too structure to generate.  On  the  other hand it   is  not too efficient on vector computers for  the iterative solution of efficient on vector computers for  the iterative solution of linear systems.  Hence,   SLAP changes   this  input    data linear systems.  Hence,   SLAP changes   this  input    data structure to the SLAP Column format  for  the iteration (but structure to the SLAP Column format  for  the iteration (but does not change it back). does not change it back). Here is an example of the  SLAP Triad   storage format for a Here is an example of the  SLAP Triad   storage format for a 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 1  2  3  4  5  6  7  8  9 10 11 1  2  3  4  5  6  7  8  9 10 11 |
| 5 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | IN       Integer IA(NELT). 5  1  1  3  1  5  5  2  3  4  2 A(JA(ICOL)) points   to the beginning  of the ICOL-th   column    in    IA and   A.      IA(JA(ICOL+1)-1), denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 \|  2  1 \|  3  5 \|  4 \|  5  1  3 is modified internally to be the SLAP Column format.  See above. Cautions: This routine will attempt to write to the Fortran logical output |
| 6 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | IN       Integer JA(NELT). 1  2  1  3  5  3  5  2  5  4  1 \| 0  0  0 44  0\| \|51  0 53  0 55\| =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the double precision array A.   In other words,  for each column in the matrix put the diagonal entry in  A.  Then put in the other non-zero  elements going down  the column (except  the diagonal) in order.   The  IA array holds the  row index for each non-zero.  The JA array holds the offsets  into the IA, A(JA(ICOL)) points   to the beginning  of the ICOL-th   column    in    IA and   A.      IA(JA(ICOL+1)-1), 1) points to  the  end of the   ICOL-th column. NELT+1,  where N is  the number of columns in  the matrix and NELT  is the number  of non-zeros in the matrix. Here is an example of the  SLAP Column  storage format for a 1  4  6    8  9   12 \| 0  0  0 44  0\| \|51  0 53  0 55\| Side Effects: is modified internally to be the SLAP Column format.  See above. Cautions: This routine will attempt to write to the Fortran logical output |
| 7 | `A` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (NELT) | IN       Double Precision A(NELT). These arrays should hold the matrix A in either the SLAP Triad format or the SLAP Column format.  See "Description", below.  If the SLAP Triad format is chosen it is changed internally to the SLAP Column format. inverse)*(SX-inverse))*(SX*M*X) = SB*B, with right preconditioning, or = SB*(M-inverse)*B, by-n double precision .le. squares system, using QR factors. RLCALC  Computes the scaled residual RL. XLCALC  Computes the solution XL. ISDGMR  User-replaceable stopping routine. The Sparse Linear Algebra Package (SLAP) utilizes two matrix data structures: 1) the  SLAP Triad  format or  2)  the SLAP Column format.  The user can hand this routine either of the of these data structures and SLAP  will figure out  which on is being used and act accordingly. =================== S L A P Triad format =================== This routine requires that the  matrix A be   stored in  the SLAP  Triad format.  In  this format only the non-zeros  are stored.  They may appear in  *ANY* order.  The user supplies 51 12 11 33 15 53 55 22 35 44 21 arrays  for  the  beginning  of each   column.   That  is, 1) points to  the  end of the   ICOL-th column. denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 \| 22 12 \| 33 53 \| 44 \| 55 15 35 is modified internally to be the SLAP Column format.  See above. Cautions: This routine will attempt to write to the Fortran logical output zero value for IUNIT.  This routine does zero IUNIT unit number. |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. |
| 9 | `NSAVE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Number of direction vectors to save and orthogonalize against. Must be greater than 1. |
| 10 | `ITOL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Flag to indicate the type of convergence criterion used. 0  Means the  iteration stops when the test described below on  the  residual RL  is satisfied.  This is the  "Natural Stopping Criteria" for this routine. Other values  of   ITOL  cause  extra,   otherwise unnecessary, computation per iteration and     are therefore  much less  efficient.  See  ISDGMR (the stop test routine) for more information. 1  Means   the  iteration stops   when the first test described below on  the residual RL  is satisfied, and there  is either right  or  no preconditioning being used. 2  Implies     that   the  user    is   using    left preconditioning, and the second stopping criterion below is used. 3  Means the  iteration stops   when  the  third test described below on Minv*Residual is satisfied, and there is either left  or no  preconditioning begin used. 11 is    often  useful  for   checking  and comparing different routines.  For this case, the  user must supply  the  "exact" solution or  a  very accurate approximation (one with  an  error much less  than norm of the difference between the iterative approximation and the user-supplied solution  divided by the  2-norm of the  user-supplied solution  is  less than TOL. Note that this requires  the  user to  set up  the "COMMON     /DSLBLK/ SOLN(LENGTH)"  in the calling routine.  The routine with this declaration should be loaded before the stop test so that the correct length is used by  the loader.  This procedure  is not standard Fortran and may not work correctly on your   system (although  it  has  worked  on every system the authors have tried).  If ITOL is not 11 then this common block is indeed standard Fortran. A*X(L)))/norm(SB*B), for right or no preconditioning, and A*X(L)))/norm(SB*B), since right or no preconditioning being used. inverse)*(B-A*X(L)))/ A*X(L)))(i)/x(i)\| SOLN))/norm(SB*SOLN). |
| 11 | `TOL` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | through a common block, COMMON /DSLBLK/ SOLN( ) INOUT    Double Precision. Convergence criterion, as described below.  If TOL is set to zero on input, then a default value of 500*(the smallest positive magnitude, machine epsilon) is used. inverse)*B), for left preconditioning, where norm() denotes the Euclidean norm, and TOL is  a positive scalar less  than one  input by the user.  If TOL equals zero  when DSDGMR is called, then a default  value  of 500*(the   smallest  positive  magnitude, machine epsilon) is used.  If the  scaling arrays SB  and SX are used, then  ideally they  should be chosen  so  that the vectors SX*X(or SX*M*X) and  SB*B have all their  components approximately equal  to  one in  magnitude.  If one wants to use the same scaling in X  and B, then  SB and SX can be the same array in the calling program. The following is a list of the other routines and their functions used by GMRES: DGMRES  Contains the matrix structure independent driver routine for GMRES. DPIGMR  Contains the main iteration loop for GMRES. DORTH   Orthogonalizes a new vector against older basis vectors. DHEQR   Computes a QR decomposition of a Hessenberg matrix. |
| 12 | `ITMAX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Maximum number of iterations.  This routine uses the default of NRMAX = ITMAX/NSAVE to determine when each restart should occur.  See the description of NRMAX and MAXL in DGMRES for a full and frightfully interesting discussion of this topic. if convergence criterion could not be achieved in iterations. |
| 13 | `ITER` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | OUT      Integer. Number of iterations required to reach convergence, or |
| 14 | `ERR` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | OUT      Double Precision. A*X(L)))/norm(SB*B), for right or no preconditioning, and inverse)*(B-A*X(L)))/ A*X(L)))/norm(SB*B), since right or no preconditioning being used. inverse)*(B-A*X(L)))/ A*X(L)))(i)/x(i)\| SOLN))/norm(SB*SOLN). |
| 15 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | OUT      Integer. Return error flag. 0 => All went well. 1 => Insufficient storage allocated for RGWK or IGWK. 2 => Routine DPIGMR failed to reduce the norm of the current residual on its last call, and so the iteration has stalled.  In this case, X equals the last computed approximation.  The user must either increase MAXL, or choose a different initial guess. 1 => Insufficient length for RGWK array. IGWK(6) contains the required minimum length of the RGWK array. 2 => Inconsistent ITOL and JPRE values. 2, RGWK(1) = RHOL, which is the norm on the left-hand-side of the relevant stopping test defined below associated with the residual for the current approximation X(L). |
| 16 | `IUNIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence.  If unit number is 0, no writing will occur. must make sure that must make sure that this logical unit is attached to a file or terminal before calling this logical unit is attached to a file or terminal before calling |
| 17 | `RWORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (LENW) | WORK    Double Precision RWORK(LENW). Double Precision array of size LENW. |
| 18 | `LENW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Length of the double precision workspace, RWORK. 1 + N*(NSAVE+7) + NSAVE*(NSAVE+3). For the recommended values of NSAVE (10), RWORK has size at least 131 + 17*N. |
| 19 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (LENIW) | INOUT    Integer IWORK(USER DEFINED >= 30). Used to hold pointers into the RWORK array. Upon return the following locations of IWORK hold information which may be of use to the user: Amount of Integer workspace actually used. Amount of Double Precision workspace actually used. 30. |
| 20 | `LENIW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. 30. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

defined by ITOL.  Letting norm() denote the Euclidean norm, ERR is defined as follows...

### Storage and workspace requirements

`RWORK`: WORK    Double Precision RWORK(LENW). Double Precision array of size LENW.

`IWORK`: INOUT    Integer IWORK(USER DEFINED >= 30). Used to hold pointers into the RWORK array. Upon return the following locations of IWORK hold information which may be of use to the user: Amount of Integer workspace actually used. Amount of Double Precision workspace actually used. 30.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::sparse::dsdgmr`. Native symbol: `dsdgmr_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::sparse::dsdgmr`
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
