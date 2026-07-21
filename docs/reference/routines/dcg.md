# DCG

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Preconditioned Conjugate Gradient Sparse Ax=b Solver. Routine to solve a symmetric positive definite linear system Ax = b using the Preconditioned Conjugate Gradient method.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, IWORK(USER DEFINED) DOUBLE PRECISION B(N), X(N), A(NELT), TOL, ERR, R(N), Z(N) DOUBLE PRECISION P(N), DZ(N), RWORK(USER DEFINED) EXTERNAL MATVEC, MSOLVE CALL DCG(N, B, X, NELT, IA, JA, A, ISYM, MATVEC, MSOLVE, $ ITOL, TOL, ITMAX, ITER, ERR, IERR, IUNIT, R, Z, P, DZ, $ RWORK, IWORK ) This routine does not care what matrix data structure is used for A and M. It simply calls the MATVEC and MSOLVE routines, with the arguments as described above. The user could write any type of structure and the appropriate MATVEC and MSOLVE routines. It is assumed that A is stored in the

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

- Canonical provider: `lin/dcg.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dcg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dcg.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DCG](https://www.netlib.org/slatec/lin/dcg.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Order of the Matrix. hand side vector and Z is the solution upon return.  NELT, IA, JA, A and NELT+1, where N is the number of columns |
| 2 | `B` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | IN       Double Precision B(N). Right-hand side vector. |
| 3 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | INOUT    Double Precision X(N). On input X is your initial guess for solution vector. On output X is the final approximate solution. |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Number of Non-Zeros stored in A. zeros in the zeros in the matrix:  (IA(NELT), JA(NELT),  A(NELT)).  For each  non-zero matrix:  (IA(NELT), JA(NELT),  A(NELT)).  For each  non-zero the  user puts   the row  and  column index   of that matrix the  user puts   the row  and  column index   of that matrix zeros  in the matrix. Here is an example of the  SLAP Column  storage format for a |
| 5 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | IN       Integer IA(NELT). JA, A  arrays in some fashion and  that M (or INV(M)) is stored  in  IWORK  and  RWORK   in  some fashion.   The SLAP routines DSDCG and DSICCG are examples of this procedure. Two  examples  of  matrix  data structures  are the: 1) SLAP Triad  format and 2) SLAP Column format. =================== S L A P Triad format =================== In  this   format only the  non-zeros are  stored.  They may appear  in *ANY* order.   The user  supplies three arrays of zero matrix  element is  placed in  the corresponding location of the A  array.  This is  an extremely easy data  structure to generate.  On  the other hand it  is  not too  efficient  on vector  computers   for the  iterative  solution  of  linear systems.  Hence, SLAP  changes this input  data structure to the SLAP   Column  format for the  iteration (but   does not change it back). Here is an example of the  SLAP Triad   storage format for a 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 1  2  3  4  5  6  7  8  9 10 11 5  1  1  3  1  5  5  2  3  4  2 1), A(JA(ICOL+1)-1) 1), A(JA(ICOL+1)-1) are  the last elements of the ICOL-th column.   Note that we are  the last elements of the ICOL-th column.   Note that we denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 1  2  5 \|  2  1 \|  3  5 \|  4 \|  5  1  3 |
| 6 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | IN       Integer JA(NELT). zero matrix  element is  placed in  the corresponding location of the A  array.  This is  an extremely easy data  structure to generate.  On  the other hand it  is  not too  efficient  on vector  computers   for the  iterative  solution  of  linear systems.  Hence, SLAP  changes this input  data structure to the SLAP   Column  format for the  iteration (but   does not change it back). Here is an example of the  SLAP Triad   storage format for a 5x5 Matrix.  Recall that the entries may appear in any order. 5x5 Matrix      SLAP Triad format for 5x5 matrix on left. 1  2  3  4  5  6  7  8  9 10 11 1  2  1  3  5  3  5  2  5  4  1 \| 0  0  0 44  0\| \|51  0 53  0 55\| =================== S L A P Column format ================== In  this format   the non-zeros are    stored counting  down columns (except  for the diagonal  entry, which must  appear first  in each "column") and are  stored in the  double pre- cision array  A. In  other  words,  for each  column  in the matrix  first put  the diagonal entry in A.  Then put in the other non-zero  elements going  down the column  (except the diagonal)  in order.  The IA array  holds the  row index for each non-zero.  The JA array  holds the offsets into the IA, 1), A(JA(ICOL+1)-1) are  the last elements of the ICOL-th column.   Note that we NELT+1, where N is the number of columns 1  4  6    8  9   12 \| 0  0  0 44  0\| \|51  0 53  0 55\| Cautions: This routine will attempt to write to the Fortran logical output |
| 7 | `A` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (NELT) | IN       Double Precision A(NELT). These arrays contain the matrix data structure for A. It could take any form.  See "Description", below, for more details. zero, denotest that A is symmetric and only the lower or upper triangle is stored. R for R for 51 12 11 33 15 53 55 22 35 44 21 arrays  for  the  beginning  of  each  column.  That  is, 1), A(JA(ICOL+1)-1) are  the last elements of the ICOL-th column.   Note that we denotes the end of a column): 5x5 Matrix      SLAP Column format for 5x5 matrix on left. 1  2  3    4  5    6  7    8    9 10 11 11 21 51 \| 22 12 \| 33 53 \| 44 \| 55 15 35 zero value for IUNIT.  This routine does zero IUNIT unit number. |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Flag to indicate symmetric storage format. zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. zero, denotest that A is symmetric and only the lower or upper triangle is stored. is a double precision array that can be used to pass necessary preconditioning information |
| 9 | `MATVEC` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | EXT      External. Name of a routine which performs the matrix vector multiply Y = A*X given A and X.  The name of the MATVEC routine must be declared external in the calling program.  The calling CALL MATVEC( N, X, Y, NELT, IA, JA, A, ISYM ) Where N is the number of unknowns, Y is the product A*X upon return X is an input vector, NELT is the number of non-zeros in the SLAP IA, JA, A storage for the matrix A. |
| 10 | `MSOLVE` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | EXT      External. CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) is an integer work array for the same purpose as RWORK. |
| 11 | `ITOL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Flag to indicate type of convergence criterion. norm of the residual divided by the 2-norm of the right-hand side is less than TOL. norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand 11 is often useful for checking and comparing different routines.  For this case, the user must supply the "exact" solution or a very accurate approximation (one with an error much less than TOL) through a common block, COMMON /DSLBLK/ SOLN( ) norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL.  Note that this requires the user to set up the "COMMON /DSLBLK/ SOLN(LENGTH)" in the calling routine. The routine with this declaration should be loaded before the stop test so that the correct length is used by the loader. This procedure is not standard Fortran and may not work correctly on your system (although it has worked on every system the authors have tried).  If ITOL is not 11 then this common block is indeed standard Fortran. |
| 12 | `TOL` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | inv is the inverse of the diagonal of A. INOUT    Double Precision. |
| 13 | `ITMAX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Maximum number of iterations. if convergence criterion could not be achieved in iterations. |
| 14 | `ITER` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | OUT      Integer. Number of iterations required to reach convergence, or |
| 15 | `ERR` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | OUT      Double Precision. |
| 16 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 4.) OUT      Integer. Return error flag. 0 => All went well. 1 => Insufficient space allocated for WORK or IWORK. 2 => Method failed to converge in ITMAX steps. 3 => Error in user input. Check input values of N, ITOL. 4 => User error tolerance set too tight. Reset to 500*D1MACH(3).  Iteration proceeded. 5 => Preconditioning matrix, M, is not positive definite.  (r,z) < 0. 6 => Matrix A is not positive definite.  (p,Ap) < 0. |
| 17 | `IUNIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IN       Integer. Unit number on which to write the error at each iteration, if this is desired for monitoring convergence.  If unit number is 0, no writing will occur. must make sure that must make sure that this logical unit is attached to a file or terminal before calling this logical unit is attached to a file or terminal before calling |
| 18 | `R` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | is supplied via hand side vector and Z is the solution upon return.  NELT, IA, JA, A and WORK     Double Precision R(N). |
| 19 | `Z` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | is supplied via WORK     Double Precision Z(N). |
| 20 | `P` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | WORK     Double Precision P(N). |
| 21 | `DZ` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | WORK     Double Precision DZ(N). Double Precision arrays used for workspace. |
| 22 | `RWORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | and IWORK arrays).  The name of the MSOLVE routine must be declared external in the calling program.  The calling is a double precision array that can be used to pass necessary preconditioning information WORK     Double Precision RWORK(USER DEFINED). Double Precision array that can be used by  MSOLVE. |
| 23 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | is an integer work array for the same purpose as RWORK. WORK     Integer IWORK(USER DEFINED). Integer array that can be used by  MSOLVE. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

defined by ITOL.

### Storage and workspace requirements

`RWORK`: and IWORK arrays).  The name of the MSOLVE routine must be declared external in the calling program.  The calling is a double precision array that can be used to pass necessary preconditioning information WORK     Double Precision RWORK(USER DEFINED). Double Precision array that can be used by  MSOLVE.

`IWORK`: is an integer work array for the same purpose as RWORK. WORK     Integer IWORK(USER DEFINED). Integer array that can be used by  MSOLVE.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::dcg`. Native symbol: `dcg_`. Declaration feature: `linear-algebra-iterative`. Provider feature: `linear-algebra`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::dcg`
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
