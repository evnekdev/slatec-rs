# DOMN

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Preconditioned Orthomin Sparse Iterative Ax=b Solver. Routine to solve a general linear system Ax = b using the Preconditioned Orthomin method.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, NSAVE, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, IWORK(USER DEFINED) DOUBLE PRECISION B(N), X(N), A(NELT), TOL, ERR, R(N), Z(N) DOUBLE PRECISION P(N,0:NSAVE), AP(N,0:NSAVE), EMAP(N,0:NSAVE) DOUBLE PRECISION DZ(N), CSAV(NSAVE), RWORK(USER DEFINED) EXTERNAL MATVEC, MSOLVE CALL DOMN(N, B, X, NELT, IA, JA, A, ISYM, MATVEC, MSOLVE, $ NSAVE, ITOL, TOL, ITMAX, ITER, ERR, IERR, IUNIT, R, $ Z, P, AP, EMAP, DZ, CSAV, RWORK, IWORK) This routine does not care what matrix data structure is used for A and M. It simply calls the MATVEC and MSOLVE routines, with the arguments as described above. The user could write any type of structure and the appropriate MATVEC and MSOLVE routines. It is assumed that A is stored in the IA, JA, A arrays in some fashion and that M (or INV(M)) is stored in IWORK and RWORK) in some fashion. The SLAP routines DSDOMN and DSLUOM are examples of this procedure. Two examples of matrix data structures are the: 1) SLAP Triad format and 2) SLAP Column format. =================== S L A P Triad format =================== In this format only the non-zeros are stored. They may appear in *ANY* order. The user supplies three arrays of length NELT, where NELT is the number of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)). For each non-zero the user puts the row and column index of that matrix element in the IA and JA arrays. The value of the non-zero matrix element is placed in the corresponding location of the A array. This is an extremely easy data structure to generate. On the other hand it is not too efficient on vector computers for the iterative solution of linear systems. Hence, SLAP changes this input data structure to the SLAP Column format for the iteration (but does not change it back). Here is an example of the SLAP Triad storage format for a 5x5 Matrix. Recall that the entries may appear in any order. 5x5 Matrix SLAP Triad format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 51 12 11 33 15 53 55 22 35 44 21 |21 22 0 0 0| IA: 5 1 1 3 1 5 5 2 3 4 2 | 0 0 33 0 35| JA: 1 2 1 3 5 3 5 2 5 4 1 | 0 0 0 44 0| |51 0 53 0 55| =================== S L A P Column format ================== In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the double pre- cision array A. In other words, for each column in the matrix first put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)),A(JA(ICOL)) are the first elements of the ICOL- th column in IA and A, and IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) are the last elements of the ICOL-th column. Note that we always have JA(N+1)=NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 |21 22 0 0 0| IA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| JA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| Cautions: This routine will attempt to write to the Fortran logical output unit IUNIT, if IUNIT .ne. 0. Thus, the user must make sure that this logical unit is attached to a file or terminal before calling this routine with a non-zero value for IUNIT. This routine does not check for the validity of a non-zero IUNIT unit number.

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

- Canonical provider: `lin/domn.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/domn.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/domn.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DOMN](https://www.netlib.org/slatec/lin/domn.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | hand side vector and Z is the solution upon return.  NELT, IA, JA, A and ISYM are defined as above.  RWORK is a double precision array that can be used to pass necessary preconditioning information and/or workspace to MSOLVE.  IWORK is an integer work array for the same purpose as RWORK. NSAVE  :IN       Integer. Number of  direction vectors to save and orthogonalize |
| 2 | `B` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `A` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (NELT) | zero, denotest that A is symmetric and only the lower or upper triangle is stored. MSOLVE :EXT      External. R for R for Z given R with the preconditioning matrix M (M is supplied via Z given R with the preconditioning matrix M (M is supplied via RWORK and IWORK arrays).  The name of the MSOLVE routine must RWORK and IWORK arrays).  The name of the MSOLVE routine must be declared external in the calling program.  The calling be declared external in the calling program.  The calling sequence to MSOLVE is: sequence to MSOLVE is: CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. MATVEC :EXT      External. Name of a routine which performs the matrix vector multiply Y = A*X given A and X.  The name of the MATVEC routine must be declared external in the calling program.  The calling sequence to MATVEC is: CALL MATVEC( N, X, Y, NELT, IA, JA, A, ISYM ) Where N is the number of unknowns, Y is the product A*X upon return X is an input vector, NELT is the number of non-zeros in the SLAP IA, JA, A storage for the matrix A. zero, denotest that A is symmetric and only the lower or upper triangle is stored. MSOLVE :EXT      External. |
| 9 | `MATVEC` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 10 | `MSOLVE` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 11 | `NSAVE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 0. ITOL   :IN       Integer. Flag to indicate type of convergence criterion. |
| 12 | `ITOL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | norm of the residual divided by the 2-norm of the right-hand side is less than TOL. norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand 11 is often useful for checking and comparing different routines.  For this case, the user must supply the "exact" |
| 13 | `TOL` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | inv is the inverse of the diagonal of A. |
| 14 | `ITMAX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 15 | `ITER` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 16 | `ERR` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 17 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 18 | `IUNIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 19 | `R` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | hand side vector and Z is the solution upon return.  NELT, IA, JA, A and ISYM are defined as above.  RWORK is a double precision array that can be used to pass necessary preconditioning information and/or workspace to MSOLVE.  IWORK is an integer work array for the same purpose as RWORK. NSAVE  :IN       Integer. Number of  direction vectors to save and orthogonalize |
| 20 | `Z` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 21 | `P` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (N, 0:NSAVE) | Array argument classified by fixed-form executable read/write analysis. |
| 22 | `AP` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (N, 0:NSAVE) | Array argument classified by fixed-form executable read/write analysis. |
| 23 | `EMAP` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (N, 0:NSAVE) | Array argument classified by fixed-form executable read/write analysis. |
| 24 | `DZ` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 25 | `CSAV` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (NSAVE) | Array argument classified by fixed-form executable read/write analysis. |
| 26 | `RWORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |
| 27 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

much less than TOL) through a common block, COMMON /DSLBLK/ SOLN( ) If ITOL=11, iteration stops when the 2-norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL.  Note that this requires the user to set up the "COMMON /DSLBLK/ SOLN(LENGTH)" in the calling routine. The routine with this declaration should be loaded before the stop test so that the correct length is used by the loader. This procedure is not standard Fortran and may not work correctly on your system (although it has worked on every system the authors have tried).  If ITOL is not 11 then this common block is indeed standard Fortran. TOL    :INOUT    Double Precision. Convergence criterion, as described above.  (Reset if IERR=4.) ITMAX  :IN       Integer. Maximum number of iterations. ITER   :OUT      Integer. Number of iterations required to reach convergence, or ITMAX+1 if convergence criterion could not be achieved in ITMAX iterations. ERR    :OUT      Double Precision. defined by ITOL. IERR   :OUT      Integer. IERR = 0 => All went well. IERR = 1 => Insufficient space allocated for WORK or IWORK. IERR = 2 => Method failed to converge in ITMAX steps. Check input values of N, ITOL. Reset to 500*D1MACH(3).  Iteration proceeded. IERR = 5 => Preconditioning matrix, M, is not positive definite.  (r,z) < 0. IERR = 6 => Breakdown of method detected. (p,Ap) < epsilon**2. IUNIT  :IN       Integer. if this is desired for monitoring convergence.  If unit number is 0, no writing will occur. R      :WORK     Double Precision R(N). Z      :WORK     Double Precision Z(N). P      :WORK     Double Precision P(N,0:NSAVE). AP     :WORK     Double Precision AP(N,0:NSAVE). EMAP   :WORK     Double Precision EMAP(N,0:NSAVE). DZ     :WORK     Double Precision DZ(N). CSAV   :WORK     Double Precision CSAV(NSAVE) Double Precision arrays used for workspace. RWORK  :WORK     Double Precision RWORK(USER DEFINED). Double Precision array that can be used for workspace in MSOLVE. IWORK  :WORK     Integer IWORK(USER DEFINED). Integer array that can be used for workspace in MSOLVE.

### Storage and workspace requirements

`RWORK`: Workspace argument classified by fixed-form executable read/write analysis.

`IWORK`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::domn`. Native symbol: `domn_`. Declaration feature: `linear-algebra-iterative`. Provider feature: `linear-algebra`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::domn`
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
