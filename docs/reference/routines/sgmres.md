# SGMRES

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Preconditioned GMRES Iterative Sparse Ax=b Solver. This routine uses the generalized minimum residual (GMRES) method with preconditioning to solve non-symmetric linear systems of the form: Ax = b.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, LRGW, IGWK(LIGW), LIGW INTEGER IWORK(USER DEFINED) REAL B(N), X(N), A(NELT), TOL, ERR, SB(N), SX(N) REAL RGWK(LRGW), RWORK(USER DEFINED) EXTERNAL MATVEC, MSOLVE CALL SGMRES(N, B, X, NELT, IA, JA, A, ISYM, MATVEC, MSOLVE, $ ITOL, TOL, ITMAX, ITER, ERR, IERR, IUNIT, SB, SX, $ RGWK, LRGW, IGWK, LIGW, RWORK, IWORK)

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

- Canonical provider: `lin/sgmres.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sgmres.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sgmres.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [SGMRES](https://www.netlib.org/slatec/lin/sgmres.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Order of the Matrix. hand side vector and Z is the solution upon return.  NELT, IA, JA, A and by-N real matrix, vectors,   SB and SX   are  diagonal scaling matrices,   and M is  a preconditioning    matrix.   It uses preconditioned  Krylov   subpace  methods  based     on  the generalized minimum residual  method (GMRES).   This routine optionally performs  either  the  full     orthogonalization version of the  GMRES  algorithm or an incomplete variant of it.  Both versions use restarting of the linear iteration by default, although the user can disable this feature. The GMRES  algorithm generates a sequence  of approximations NELT+1, where  N  is the number of columns in  the |
| 2 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | IN       Real B(N). Right-hand side vector. A*X(L)))/norm(SB*B), for right or no preconditioning, and A*X(L)))/norm(SB*B), since right or no preconditioning being used. A*X(L)))(i)/x(i)\| vectors,   SB and SX   are  diagonal scaling matrices,   and M is  a preconditioning    matrix.   It uses preconditioned  Krylov   subpace  methods  based     on  the generalized minimum residual  method (GMRES).   This routine optionally performs  either  the  full     orthogonalization version of the  GMRES  algorithm or an incomplete variant of it.  Both versions use restarting of the linear iteration by default, although the user can disable this feature. The GMRES  algorithm generates a sequence  of approximations A*X(L))) .le. TOL*norm(SB*B), for right preconditioning, or |
| 3 | `X` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | INOUT    Real X(N). On input X is your initial guess for the solution vector. On output X is the final approximate solution. SOLN))/norm(SB*SOLN). X0 is to be found (where, X0 is the initial guess).  The default value of MAXL is 10. vectors,   SB and SX   are  diagonal scaling matrices,   and M is  a preconditioning    matrix.   It uses preconditioned  Krylov   subpace  methods  based     on  the generalized minimum residual  method (GMRES).   This routine optionally performs  either  the  full     orthogonalization version of the  GMRES  algorithm or an incomplete variant of it.  Both versions use restarting of the linear iteration by default, although the user can disable this feature. The GMRES  algorithm generates a sequence  of approximations to the  true solution of the above  linear system.  The convergence criteria for stopping the  iteration is based on |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Number of Non-Zeros stored in A. is  the number  of is  the number  of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)).  For non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)).  For each non-zero the user puts the row and column index of that each non-zero the user puts the row and column index of that matrix element  in the IA and  JA arrays.  The  value of the matrix element  in the IA and  JA arrays.  The  value of the non-zero   matrix  element is  placed  in  the corresponding non-zero   matrix  element is  placed  in  the corresponding location of the A array.   This is  an  extremely  easy data location of the A array.   This is  an  extremely  easy data structure to generate.  On  the  other hand it   is  not too structure to generate.  On  the  other hand it   is  not too efficient on vector computers for  the iterative solution of efficient on vector computers for  the iterative solution of linear systems.  Hence,   SLAP changes   this  input    data linear systems.  Hence,   SLAP changes   this  input    data structure to the SLAP Column format  for  the iteration (but structure to the SLAP Column format  for  the iteration (but does not change it back). does not change it back). Here is an example of the  SLAP Triad   storage format for a Here is an example of the  SLAP Triad   storage format for a 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 1  2  3  4  5  6  7  8  9 10 11 1  2  3  4  5  6  7  8  9 10 11 zeros in the matrix. Here is an example of the  SLAP Column  storage format for a |
| 5 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | IN       Integer IA(NELT). JA, A  arrays in some fashion and  that M (or INV(M)) is stored  in  IWORK  and  RWORK   in  some fashion.   The SLAP routines SSDCG and SSICCG are examples of this procedure. Two  examples  of  matrix  data structures  are the: 1) SLAP Triad  format and 2) SLAP Column format. =================== S L A P Triad format =================== This routine requires that the  matrix A be   stored in  the SLAP  Triad format.  In  this format only the non-zeros  are stored.  They may appear in  *ANY* order.  The user supplies 5  1  1  3  1  5  5  2  3  4  2 zero. The JA array holds the offsets into the IA, A arrays for the beginning of   each    column.    That  is,    IA(JA(ICOL)), 1),  A(JA(ICOL+1)-1) points to the 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 \|  2  1 \|  3  5 \|  4 \|  5  1  3 |
| 6 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | IN       Integer JA(NELT). 1  2  1  3  5  3  5  2  5  4  1 \| 0  0  0 44  0\| \|51  0 53  0 55\| =================== S L A P Column format ================== This routine  requires that  the matrix A  be stored in  the SLAP Column format.  In this format the non-zeros are stored counting down columns (except for  the diagonal entry, which must appear first in each  "column")  and are stored  in the real array A.  In other words, for each column in the matrix th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have NELT+1, where  N  is the number of columns in  the 1  4  6    8  9   12 \| 0  0  0 44  0\| \|51  0 53  0 55\| Cautions: This routine will attempt to write to the Fortran logical output |
| 7 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NELT) | IN       Real A(NELT). These arrays contain the matrix data structure for A. It could take any form.  See "Description", below, for more details. zero, denotes that A is symmetric and only the lower or upper triangle is stored. r for z given r with the preconditioning matrix M (M is supplied via inverse)*(SX-inverse))*(SX*M*X) = SB*B, with right preconditioning, or by-N real matrix, squares system, using QR factors. SRLCAL  Computes the scaled residual RL. SXLCAL  Computes the solution XL. ISSGMR  User-replaceable stopping routine. This routine does  not care  what matrix data   structure is used for  A and M.  It simply   calls  the MATVEC and MSOLVE routines, with  the arguments as  described above.  The user could write any type of structure and the appropriate MATVEC and MSOLVE routines.  It is assumed  that A is stored in the 51 12 11 33 15 53 55 22 35 44 21 zero elements going down   the  column (except  the diagonal)  in th column in 1),  A(JA(ICOL+1)-1) points to the end  of   the ICOL-th  column.  Note   that  we  always have denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 \| 22 12 \| 33 53 \| 44 \| 55 15 35 zero value for IUNIT.  This routine does zero IUNIT unit number. |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. zero, denotes that A is symmetric and only the lower or upper triangle is stored. is a real array that can be used to pass necessary preconditioning information and/or |
| 9 | `MATVEC` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | EXT      External. Name of a routine which performs the matrix vector multiply Y = A*X given A and X.  The name of the MATVEC routine must be declared external in the calling program.  The calling CALL MATVEC(N, X, Y, NELT, IA, JA, A, ISYM) where N is the number of unknowns, Y is the product A*X upon return, X is an input vector, and NELT is the number of non-zeros in the SLAP IA, JA, A storage for the matrix A. |
| 10 | `MSOLVE` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | EXT      External. CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) is an integer work array for the same purpose as RWORK. |
| 11 | `ITOL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Flag to indicate the type of convergence criterion used. 0  Means the  iteration stops when the test described below on  the  residual RL  is satisfied.  This is the  "Natural Stopping Criteria" for this routine. Other values  of   ITOL  cause  extra,   otherwise unnecessary, computation per iteration and     are therefore  much less  efficient.  See  ISSGMR (the stop test routine) for more information. 1  Means   the  iteration stops   when the first test described below on  the residual RL  is satisfied, and there  is either right  or  no preconditioning being used. 2  Implies     that   the  user    is   using    left preconditioning, and the second stopping criterion below is used. 3  Means the  iteration stops   when  the  third test described below on Minv*Residual is satisfied, and there is either left  or no  preconditioning being used. 11 is    often  useful  for   checking  and comparing different routines.  For this case, the  user must supply  the  "exact" solution or  a  very accurate approximation (one with  an  error much less  than norm of the difference between the iterative approximation and the user-supplied solution  divided by the  2-norm of the  user-supplied solution  is  less than TOL. Note that this requires  the  user to  set up  the "COMMON     /SSLBLK/ SOLN(LENGTH)"  in the calling routine.  The routine with this declaration should be loaded before the stop test so that the correct length is used by  the loader.  This procedure  is not standard Fortran and may not work correctly on your   system (although  it  has  worked  on every system the authors have tried).  If ITOL is not 11 then this common block is indeed standard Fortran. A*X(L)))/norm(SB*B), for right or no preconditioning, and A*X(L)))/norm(SB*B), since right or no preconditioning being used. inverse)*(B-A*X(L)))/ A*X(L)))(i)/x(i)\| SOLN))/norm(SB*SOLN). |
| 12 | `TOL` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | through a common block, COMMON /SSLBLK/ SOLN( ) INOUT    Real. Convergence criterion, as described below.  If TOL is set to zero on input, then a default value of 500*(the smallest positive magnitude, machine epsilon) is used. inverse)*B), for left preconditioning, where norm() denotes the Euclidean norm, and TOL is  a positive scalar less  than one  input by the user.  If TOL equals zero  when SGMRES is called, then a default  value  of 500*(the   smallest  positive  magnitude, machine epsilon) is used.  If the  scaling arrays SB  and SX are used, then  ideally they  should be chosen  so  that the vectors SX*X(or SX*M*X) and  SB*B have all their  components approximately equal  to  one in  magnitude.  If one wants to use the same scaling in X  and B, then  SB and SX can be the same array in the calling program. The following is a list of the other routines and their functions used by SGMRES: SPIGMR  Contains the main iteration loop for GMRES. SORTH   Orthogonalizes a new vector against older basis vectors. SHEQR   Computes a QR decomposition of a Hessenberg matrix. |
| 13 | `ITMAX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | DUMMY    Integer. Maximum number of iterations in most SLAP routines.  In this routine this does not make sense.  The maximum number MAXL*(NRMAX+1). See IGWK for definitions of MAXL and NRMAX. if convergence criterion could not be achieved in iterations. |
| 14 | `ITER` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | OUT      Integer. Number of iterations required to reach convergence, or |
| 15 | `ERR` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | OUT      Real. A*X(L)))/norm(SB*B), for right or no preconditioning, and inverse)*(B-A*X(L)))/ A*X(L)))/norm(SB*B), since right or no preconditioning being used. inverse)*(B-A*X(L)))/ A*X(L)))(i)/x(i)\| SOLN))/norm(SB*SOLN). |
| 16 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | OUT      Integer. Return error flag. 0 => All went well. 1 => Insufficient storage allocated for 2 => Routine SGMRES failed to reduce the norm of the current residual on its last call, and so the iteration has stalled.  In this case, X equals the last computed approximation.  The user must either increase MAXL, or choose a different initial guess. 1 => Insufficient length for RGWK array. 2 => Illegal value of ITOL, or ITOL and JPRE values are inconsistent. 2, RGWK(1) = RHOL, which is the norm on the left-hand-side of the relevant stopping test defined below associated with the residual for the current approximation X(L). |
| 17 | `IUNIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence.  If unit number is 0, no writing will occur. must make sure that must make sure that this logical unit is attached to a file or terminal before calling this logical unit is attached to a file or terminal before calling |
| 18 | `SB` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | A*X(L)))/norm(SB*B), for right or no preconditioning, and inverse)*(B-A*X(L)))/ inverse)*B), for left preconditioning. A*X(L)))/norm(SB*B), since right or no preconditioning being used. inverse)*(B-A*X(L)))/ inverse)*B), since left preconditioning is being used. SOLN))/norm(SB*SOLN). IN       Real SB(N). Array of length N containing scale factors for the right hand side vector B.  If JSCAL.eq.0 (see below), SB need not be supplied. are to be used. are not used and the algorithm 1 and SX(I) = 1. JSCAL = 1 =>  Only SX is used, and the algorithm 1. JSCAL = 2 =>  Only SB is used, and the algorithm are used. inverse)*(SX-inverse))*(SX*M*X) = SB*B, with right preconditioning, or inverse)*A*(SX-inverse))*(SX*X) = SB*(M-inverse)*B, A*X(L))) .le. TOL*norm(SB*B), for right preconditioning, or inverse)*(B-A*X(L))) .le. inverse)*B), for left preconditioning, where norm() denotes the Euclidean norm, and TOL is  a positive scalar less  than one  input by the user.  If TOL equals zero  when SGMRES is called, then a default  value  of 500*(the   smallest  positive  magnitude, machine epsilon) is used.  If the  scaling arrays SB  and SX are used, then  ideally they  should be chosen  so  that the vectors SX*X(or SX*M*X) and  SB*B have all their  components approximately equal  to  one in  magnitude.  If one wants to use the same scaling in X  and B, then  SB and SX can be the same array in the calling program. The following is a list of the other routines and their functions used by SGMRES: SPIGMR  Contains the main iteration loop for GMRES. SORTH   Orthogonalizes a new vector against older basis vectors. SHEQR   Computes a QR decomposition of a Hessenberg matrix. |
| 19 | `SX` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | IN       Real SX(N). Array of length N containing scale factors for the solution vector X.  If JSCAL.eq.0 (see below), SX need not be supplied.  SB and SX can be the same array in the calling program if desired. are to be used. are not used and the algorithm 1. are used. |
| 20 | `RGWK` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (LRGW) | or IGWK. INOUT    Real RGWK(LRGW). Real array used for workspace by SGMRES. RHOL.  See IERR for definition of RHOL. |
| 21 | `LRGW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Length of the real workspace, RGWK. 1 + N*(MAXL+6) + MAXL*(MAXL+3). See below for definition of MAXL. For the default values, RGWK has size at least 131 + 16*N. |
| 22 | `IGWK` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (LIGW) | contains the required minimum length of the RGWK array. INOUT    Integer IGWK(LIGW). The following IGWK parameters should be set by the user before calling this routine. MAXL.  Maximum dimension of Krylov subspace in KMP.  Maximum number of previous Krylov basis vectors to which each new basis vector is made orthogonal. The default value of KMP is MAXL. JSCAL.  Flag indicating whether the scaling JPRE.  Flag indicating whether preconditioning is being used. JPRE = 0  =>  There is no preconditioning. JPRE > 0  =>  There is preconditioning on the right only, and the solver will call routine MSOLVE. JPRE < 0  =>  There is preconditioning on the left only, and the solver will call routine MSOLVE. NRMAX.  Maximum number of restarts of the Krylov iteration.  The default value of NRMAX = 10. MLWK.  Required minimum length of RGWK array. NMS.  The total number of calls to MSOLVE. 20. |
| 23 | `LIGW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. 20. |
| 24 | `RWORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | and IWORK arrays.  The name of the MSOLVE routine must be declared external in the calling program.  The calling is a real array that can be used to pass necessary preconditioning information and/or WORK     Real RWORK(USER DEFINED). Real array that can be used for workspace in MSOLVE. |
| 25 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | is an integer work array for the same purpose as RWORK. 1,  then no restarts are performed (in this case, NRMAX is set to zero internally). The following IWORK parameters are diagnostic information made available to the user after this routine completes. WORK     Integer IWORK(USER DEFINED). Integer array that can be used for workspace in MSOLVE. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

defined by ITOL.  Letting norm() denote the Euclidean norm, ERR is defined as follows..

### Storage and workspace requirements

`RWORK`: and IWORK arrays.  The name of the MSOLVE routine must be declared external in the calling program.  The calling is a real array that can be used to pass necessary preconditioning information and/or WORK     Real RWORK(USER DEFINED). Real array that can be used for workspace in MSOLVE.

`IWORK`: is an integer work array for the same purpose as RWORK. 1,  then no restarts are performed (in this case, NRMAX is set to zero internally). The following IWORK parameters are diagnostic information made available to the user after this routine completes. WORK     Integer IWORK(USER DEFINED). Integer array that can be used for workspace in MSOLVE.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::sgmres`. Native symbol: `sgmres_`. Declaration feature: `linear-algebra-iterative`. Provider feature: `linear-algebra`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32),mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::sgmres`
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
