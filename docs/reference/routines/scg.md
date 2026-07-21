# SCG

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Preconditioned Conjugate Gradient Sparse Ax=b Solver. Routine to solve a symmetric positive definite linear system Ax = b using the Preconditioned Conjugate Gradient method.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, IWORK(USER DEFINED) REAL B(N), X(N), A(NELT), TOL, ERR, R(N), Z(N) REAL P(N), DZ(N), RWORK(USER DEFINED) EXTERNAL MATVEC, MSOLVE CALL SCG(N, B, X, NELT, IA, JA, A, ISYM, MATVEC, MSOLVE, $ ITOL, TOL, ITMAX, ITER, ERR, IERR, IUNIT, R, Z, P, DZ, $ RWORK, IWORK ) This routine does not care what matrix data structure is used for A and M. It simply calls the MATVEC and MSOLVE routines, with the arguments as described above. The user could write any type of structure and the appropriate MATVEC and MSOLVE routines. It is assumed that A is stored in the IA, JA, A arrays in some fashion and that M (or INV(M)) is stored in IWORK and RWORK in some fashion. The SLAP routines SSDCG and SSICCG are examples of this procedure. Two examples of matrix data structures are the: 1) SLAP Triad format and 2) SLAP Column format. =================== S L A P Triad format =================== In this format only the non-zeros are stored. They may appear in *ANY* order. The user supplies three arrays of length NELT, where NELT is the number of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)). For each non-zero the user puts the row and column index of that matrix element in the IA and JA arrays. The value of the non-zero matrix element is placed in the corresponding location of the A array. This is an extremely easy data structure to generate. On the other hand it is not too efficient on vector computers for the iterative solution of linear systems. Hence, SLAP changes this input data structure to the SLAP Column format for the iteration (but does not change it back). Here is an example of the SLAP Triad storage format for a 5x5 Matrix. Recall that the entries may appear in any order. 5x5 Matrix SLAP Triad format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 51 12 11 33 15 53 55 22 35 44 21 |21 22 0 0 0| IA: 5 1 1 3 1 5 5 2 3 4 2 | 0 0 33 0 35| JA: 1 2 1 3 5 3 5 2 5 4 1 | 0 0 0 44 0| |51 0 53 0 55| =================== S L A P Column format ================== In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the end of the ICOL-th column. Note that we always have JA(N+1) = NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 |21 22 0 0 0| IA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| JA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| Cautions: This routine will attempt to write to the Fortran logical output unit IUNIT, if IUNIT .ne. 0. Thus, the user must make sure that this logical unit is attached to a file or terminal before calling this routine with a non-zero value for IUNIT. This routine does not check for the validity of a non-zero IUNIT unit number.

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
- GAMS classifications: `D2B4`
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

- Canonical provider: `lin/scg.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/scg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/scg.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [SCG](https://www.netlib.org/slatec/lin/scg.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | hand side vector and Z is the solution upon return.  NELT, IA, JA, A and ISYM are defined as above.  RWORK is a real array that can be used to pass necessary preconditioning information and/or workspace to MSOLVE.  IWORK is an integer work array for the same purpose as RWORK. ITOL   :IN       Integer. Flag to indicate type of convergence criterion. |
| 2 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (NELT) | zero, denotest that A is symmetric and only the lower or upper triangle is stored. MSOLVE :EXT      External. R for R for Z given R with the preconditioning matrix M (M is supplied via Z given R with the preconditioning matrix M (M is supplied via RWORK and IWORK arrays).  The name of the MSOLVE routine must RWORK and IWORK arrays).  The name of the MSOLVE routine must be declared external in the calling program.  The calling be declared external in the calling program.  The calling sequence to MSOLVE is: sequence to MSOLVE is: CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. MATVEC :EXT      External. Name of a routine which performs the matrix vector multiply Y = A*X given A and X.  The name of the MATVEC routine must be declared external in the calling program.  The calling sequence to MATVEC is: CALL MATVEC( N, X, Y, NELT, IA, JA, A, ISYM ) Where N is the number of unknowns, Y is the product A*X upon return X is an input vector, NELT is the number of non-zeros in the SLAP IA, JA, A storage for the matrix A. zero, denotest that A is symmetric and only the lower or upper triangle is stored. MSOLVE :EXT      External. |
| 9 | `MATVEC` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 10 | `MSOLVE` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 11 | `ITOL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | norm of the residual divided by the 2-norm of the right-hand side is less than TOL. norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand 11 is often useful for checking and comparing different routines.  For this case, the user must supply the "exact" |
| 12 | `TOL` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | inv is the inverse of the diagonal of A. |
| 13 | `ITMAX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 14 | `ITER` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 15 | `ERR` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 16 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 17 | `IUNIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 18 | `R` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | hand side vector and Z is the solution upon return.  NELT, IA, JA, A and ISYM are defined as above.  RWORK is a real array that can be used to pass necessary preconditioning information and/or workspace to MSOLVE.  IWORK is an integer work array for the same purpose as RWORK. ITOL   :IN       Integer. Flag to indicate type of convergence criterion. |
| 19 | `Z` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 20 | `P` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 21 | `DZ` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 22 | `RWORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |
| 23 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

much less than TOL) through a common block, COMMON /SSLBLK/ SOLN( ) If ITOL=11, iteration stops when the 2-norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL.  Note that this requires the user to set up the "COMMON /SSLBLK/ SOLN(LENGTH)" in the calling routine. The routine with this declaration should be loaded before the stop test so that the correct length is used by the loader. This procedure is not standard Fortran and may not work correctly on your system (although it has worked on every system the authors have tried).  If ITOL is not 11 then this common block is indeed standard Fortran. TOL    :INOUT    Real. Convergence criterion, as described above.  (Reset if IERR=4.) ITMAX  :IN       Integer. Maximum number of iterations. ITER   :OUT      Integer. Number of iterations required to reach convergence, or ITMAX+1 if convergence criterion could not be achieved in ITMAX iterations. ERR    :OUT      Real. defined by ITOL. IERR   :OUT      Integer. IERR = 0 => All went well. IERR = 1 => Insufficient space allocated for WORK or IWORK. IERR = 2 => Method failed to converge in ITMAX steps. Check input values of N, ITOL. Reset to 500*R1MACH(3).  Iteration proceeded. IERR = 5 => Preconditioning matrix, M, is not positive definite.  (r,z) < 0. IERR = 6 => Matrix A is not positive definite.  (p,Ap) < 0. IUNIT  :IN       Integer. if this is desired for monitoring convergence.  If unit number is 0, no writing will occur. R      :WORK     Real R(N). Z      :WORK     Real Z(N). P      :WORK     Real P(N). DZ     :WORK     Real DZ(N). Real arrays used for workspace. RWORK  :WORK     Real RWORK(USER DEFINED). Real array that can be used by  MSOLVE. IWORK  :WORK     Integer IWORK(USER DEFINED). Integer array that can be used by  MSOLVE.

### Storage and workspace requirements

`RWORK`: Workspace argument classified by fixed-form executable read/write analysis.

`IWORK`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::scg`. Native symbol: `scg_`. Declaration feature: `linear-algebra-iterative`. Provider feature: `linear-algebra`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32),mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::scg`
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
