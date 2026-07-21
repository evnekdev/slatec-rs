# DBCG

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Preconditioned BiConjugate Gradient Sparse Ax = b Solver. Routine to solve a Non-Symmetric linear system Ax = b using the Preconditioned BiConjugate Gradient method.

## Description

Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, ITOL, ITMAX INTEGER ITER, IERR, IUNIT, IWORK(USER DEFINED) DOUBLE PRECISION B(N), X(N), A(NELT), TOL, ERR, R(N), Z(N), P(N) DOUBLE PRECISION RR(N), ZZ(N), PP(N), DZ(N) DOUBLE PRECISION RWORK(USER DEFINED) EXTERNAL MATVEC, MTTVEC, MSOLVE, MTSOLV CALL DBCG(N, B, X, NELT, IA, JA, A, ISYM, MATVEC, MTTVEC, $ MSOLVE, MTSOLV, ITOL, TOL, ITMAX, ITER, ERR, IERR, IUNIT, $ R, Z, P, RR, ZZ, PP, DZ, RWORK, IWORK) This routine does not care what matrix data structure is used for A and M. It simply calls MATVEC, MTTVEC, MSOLVE, MTSOLV routines, with arguments as above. The user could write any

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

- Canonical provider: `lin/dbcg.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dbcg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dbcg.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DBCG](https://www.netlib.org/slatec/lin/dbcg.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | hand side vector, and Z is the solution upon return.  NELT,  IA, JA, A and  ISYM define the SLAP  matrix  data structure: see Description, below.  RWORK is a  double precision array that can be used to pass necessary preconditioning information and/ or workspace to MSOLVE.  IWORK is an integer work array for the same purpose as RWORK. MTSOLV :EXT      External. hand side vector, and ZZ is the solution upon return.  NELT, IA, JA, A and  ISYM define the SLAP  matrix  data structure: see Description, below.  RWORK is a  double precision array that can be used to pass necessary preconditioning information and/ or workspace to MTSOLV.  IWORK is an integer work array for the same purpose as RWORK. ITOL   :IN       Integer. Flag to indicate type of convergence criterion. |
| 2 | `B` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 3 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `NELT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `IA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `JA` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `A` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (NELT) | R  for Z R  for Z given R with the preconditioning matrix M (M is supplied via given R with the preconditioning matrix M (M is supplied via RWORK  and IWORK arrays).   The name  of  the MSOLVE routine RWORK  and IWORK arrays).   The name  of  the MSOLVE routine must be declared  external  in the  calling   program.   The must be declared  external  in the  calling   program.   The calling sequence of MSOLVE is: calling sequence of MSOLVE is: CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RWORK, IWORK) RR for RR for ZZ given RR with the preconditioning matrix M (M is supplied ZZ given RR with the preconditioning matrix M (M is supplied via RWORK and IWORK arrays).  The name of the MTSOLV routine via RWORK and IWORK arrays).  The name of the MTSOLV routine must be declared external in the calling program.  The call- must be declared external in the calling program.  The call- ing sequence to MTSOLV is: ing sequence to MTSOLV is: CALL MTSOLV(N, RR, ZZ, NELT, IA, JA, A, ISYM, RWORK, IWORK) CALL MTSOLV(N, RR, ZZ, NELT, IA, JA, A, ISYM, RWORK, IWORK) |
| 8 | `ISYM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | zero entries of the matrix are stored. 1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. MATVEC :EXT      External. Name of a routine which  performs the matrix vector multiply operation  Y = A*X  given A and X.  The  name of  the MATVEC routine must  be declared external  in the  calling program. The calling sequence of MATVEC is: CALL MATVEC( N, X, Y, NELT, IA, JA, A, ISYM ) Where N is the number of unknowns, Y is the product A*X upon return,  X is an input  vector.  NELT, IA,  JA,  A and  ISYM define the SLAP matrix data structure: see Description,below. MTTVEC :EXT      External. Name of a routine which performs the matrix transpose vector multiply y = A'*X given A and X (where ' denotes transpose). The name of the MTTVEC routine must be declared external  in the calling program.  The calling sequence to MTTVEC is  the same as that for MTTVEC, viz.: CALL MTTVEC( N, X, Y, NELT, IA, JA, A, ISYM ) Where N  is the number  of unknowns, Y is the   product A'*X upon return, X is an input vector.  NELT, IA, JA, A and ISYM define the SLAP matrix data structure: see Description,below. MSOLVE :EXT      External. |
| 9 | `MATVEC` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 10 | `MTTVEC` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 11 | `MSOLVE` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 12 | `MTSOLV` | `callback` | `callback` | `INTEGER` | `reviewed unsafe extern callback function pointer` | scalar | Callback argument classified by fixed-form executable read/write analysis. |
| 13 | `ITOL` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | norm of the residual divided by the 2-norm of the right-hand side is less than TOL. norm of M-inv times the residual divided by the 2-norm of M-inv times the right hand 11 is often useful for checking and comparing different routines.  For this case, the user must supply the "exact" |
| 14 | `TOL` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | inv is the inverse of the diagonal of A. |
| 15 | `ITMAX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 16 | `ITER` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 17 | `ERR` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 18 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 19 | `IUNIT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 20 | `R` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | hand side vector, and Z is the solution upon return.  NELT,  IA, JA, A and  ISYM define the SLAP  matrix  data structure: see Description, below.  RWORK is a  double precision array that can be used to pass necessary preconditioning information and/ or workspace to MSOLVE.  IWORK is an integer work array for the same purpose as RWORK. MTSOLV :EXT      External. |
| 21 | `Z` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 22 | `P` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 23 | `RR` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | hand side vector, and ZZ is the solution upon return.  NELT, IA, JA, A and  ISYM define the SLAP  matrix  data structure: see Description, below.  RWORK is a  double precision array that can be used to pass necessary preconditioning information and/ or workspace to MTSOLV.  IWORK is an integer work array for the same purpose as RWORK. ITOL   :IN       Integer. Flag to indicate type of convergence criterion. |
| 24 | `ZZ` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | RR for ZZ given RR with the preconditioning matrix M (M is supplied via RWORK and IWORK arrays).  The name of the MTSOLV routine must be declared external in the calling program.  The call- ing sequence to MTSOLV is: CALL MTSOLV(N, RR, ZZ, NELT, IA, JA, A, ISYM, RWORK, IWORK) |
| 25 | `PP` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 26 | `DZ` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (N) | Array argument classified by fixed-form executable read/write analysis. |
| 27 | `RWORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |
| 28 | `IWORK` | `workspace` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

much less than TOL) through a common block, COMMON /DSLBLK/ SOLN( ) If ITOL=11, iteration stops when the 2-norm of the difference between the iterative approximation and the user-supplied solution divided by the 2-norm of the user-supplied solution is less than TOL.  Note that this requires the user to set up the "COMMON /DSLBLK/ SOLN(LENGTH)" in the calling routine. The routine with this declaration should be loaded before the stop test so that the correct length is used by the loader. This procedure is not standard Fortran and may not work correctly on your system (although it has worked on every system the authors have tried).  If ITOL is not 11 then this common block is indeed standard Fortran. TOL    :INOUT    Double Precision. Convergence criterion, as described above.  (Reset if IERR=4.) ITMAX  :IN       Integer. Maximum number of iterations. ITER   :OUT      Integer. Number of iterations required to reach convergence, or ITMAX+1 if convergence criterion could not be achieved in ITMAX iterations. ERR    :OUT      Double Precision. defined by ITOL. IERR   :OUT      Integer. IERR = 0 => All went well. IERR = 1 => Insufficient space allocated for WORK or IWORK. IERR = 2 => Method failed to converge in ITMAX steps. Check input values of N, ITOL. Reset to 500*D1MACH(3).  Iteration proceeded. IERR = 5 => Preconditioning matrix, M, is not positive definite.  (r,z) < 0. IERR = 6 => Matrix A is not positive definite.  (p,Ap) < 0. IUNIT  :IN       Integer. if this is desired for monitoring convergence.  If unit number is 0, no writing will occur. R      :WORK     Double Precision R(N). Z      :WORK     Double Precision Z(N). P      :WORK     Double Precision P(N). RR     :WORK     Double Precision RR(N). ZZ     :WORK     Double Precision ZZ(N). PP     :WORK     Double Precision PP(N). DZ     :WORK     Double Precision DZ(N). Double Precision arrays used for workspace. RWORK  :WORK     Double Precision RWORK(USER DEFINED). Double Precision array that can be used for workspace in MSOLVE and MTSOLV. IWORK  :WORK     Integer IWORK(USER DEFINED). Integer array that can be used for workspace in MSOLVE and MTSOLV.

### Storage and workspace requirements

`RWORK`: Workspace argument classified by fixed-form executable read/write analysis.

`IWORK`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::dbcg`. Native symbol: `dbcg_`. Declaration feature: `linear-algebra-iterative`. Provider feature: `linear-algebra`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::sparse::callbacks::dbcg`
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
