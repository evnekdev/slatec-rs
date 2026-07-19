# DPIGMR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Internal routine for DGMRES.

## Description

This routine solves the linear system A * Z = R0 using a scaled preconditioned version of the generalized minimum residual method. An initial guess of Z = 0 is assumed. *Usage: INTEGER N, JSCAL, MAXL, MAXLP1, KMP, NRSTS, JPRE, NMSL, LGMR INTEGER IPAR(USER DEFINED), NRMAX, ITOL, NELT, IA(NELT), JA(NELT) INTEGER ISYM, IUNIT, IFLAG DOUBLE PRECISION R0(N), SR(N), SZ(N), Z(N), V(N,MAXLP1), $ HES(MAXLP1,MAXL), Q(2*MAXL), RPAR(USER DEFINED), $ WK(N), DL(N), RHOL, B(N), BNRM, X(N), XL(N), $ TOL, A(NELT), ERR EXTERNAL MATVEC, MSOLVE CALL DPIGMR(N, R0, SR, SZ, JSCAL, MAXL, MAXLP1, KMP, $ NRSTS, JPRE, MATVEC, MSOLVE, NMSL, Z, V, HES, Q, LGMR, $ RPAR, IPAR, WK, DL, RHOL, NRMAX, B, BNRM, X, XL, $ ITOL, TOL, NELT, IA, JA, A, ISYM, IUNIT, IFLAG, ERR) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. R0 :IN Double Precision R0(N) R0 = the right hand side of the system A*Z = R0. R0 is also used as workspace when computing the final approximation. (R0 is the same as V(*,MAXL+1) in the call to DPIGMR.) SR :IN Double Precision SR(N) SR is a vector of length N containing the non-zero elements of the diagonal scaling matrix for R0. SZ :IN Double Precision SZ(N) SZ is a vector of length N containing the non-zero elements of the diagonal scaling matrix for Z. JSCAL :IN Integer A flag indicating whether arrays SR and SZ are used. JSCAL=0 means SR and SZ are not used and the algorithm will perform as if all SR(i) = 1 and SZ(i) = 1. JSCAL=1 means only SZ is used, and the algorithm performs as if all SR(i) = 1. JSCAL=2 means only SR is used, and the algorithm performs as if all SZ(i) = 1. JSCAL=3 means both SR and SZ are used. MAXL :IN Integer The maximum allowable order of the matrix H. MAXLP1 :IN Integer MAXPL1 = MAXL + 1, used for dynamic dimensioning of HES. KMP :IN Integer The number of previous vectors the new vector VNEW must be made orthogonal to. (KMP .le. MAXL) NRSTS :IN Integer Counter for the number of restarts on the current call to DGMRES. If NRSTS .gt. 0, then the residual R0 is already scaled, and so scaling of it is not necessary. JPRE :IN Integer Preconditioner type flag. MATVEC :EXT External. Name of a routine which performs the matrix vector multiply Y = A*X given A and X. The name of the MATVEC routine must be declared external in the calling program. The calling sequence to MATVEC is: CALL MATVEC(N, X, Y, NELT, IA, JA, A, ISYM) where N is the number of unknowns, Y is the product A*X upon return, X is an input vector, and NELT is the number of non-zeros in the SLAP IA, JA, A storage for the matrix A. ISYM is a flag which, if non-zero, denotes that A is symmetric and only the lower or upper triangle is stored. MSOLVE :EXT External. Name of the routine which solves a linear system Mz = r for z given r with the preconditioning matrix M (M is supplied via RPAR and IPAR arrays. The name of the MSOLVE routine must be declared external in the calling program. The calling sequence to MSOLVE is: CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RPAR, IPAR) Where N is the number of unknowns, R is the right-hand side vector and Z is the solution upon return. NELT, IA, JA, A and ISYM are defined as below. RPAR is a double precision array that can be used to pass necessary preconditioning information and/or workspace to MSOLVE. IPAR is an integer work array for the same purpose as RPAR. NMSL :OUT Integer The number of calls to MSOLVE. Z :OUT Double Precision Z(N) The final computed approximation to the solution of the system A*Z = R0. V :OUT Double Precision V(N,MAXLP1) The N by (LGMR+1) array containing the LGMR orthogonal vectors V(*,1) to V(*,LGMR). HES :OUT Double Precision HES(MAXLP1,MAXL) The upper triangular factor of the QR decomposition of the (LGMR+1) by LGMR upper Hessenberg matrix whose entries are the scaled inner-products of A*V(*,I) and V(*,K). Q :OUT Double Precision Q(2*MAXL) A double precision array of length 2*MAXL containing the components of the Givens rotations used in the QR decomposition of HES. It is loaded in DHEQR and used in DHELS. LGMR :OUT Integer The number of iterations performed and the current order of the upper Hessenberg matrix HES. RPAR :IN Double Precision RPAR(USER DEFINED) Double Precision workspace passed directly to the MSOLVE routine. IPAR :IN Integer IPAR(USER DEFINED) Integer workspace passed directly to the MSOLVE routine. WK :IN Double Precision WK(N) A double precision work array of length N used by routines MATVEC and MSOLVE. DL :INOUT Double Precision DL(N) On input, a double precision work array of length N used for calculation of the residual norm RHO when the method is incomplete (KMP.lt.MAXL), and/or when using restarting. On output, the scaled residual vector RL. It is only loaded when performing restarts of the Krylov iteration. RHOL :OUT Double Precision A double precision scalar containing the norm of the final residual. NRMAX :IN Integer The maximum number of restarts of the Krylov iteration. NRMAX .gt. 0 means restarting is active, while NRMAX = 0 means restarting is not being used. B :IN Double Precision B(N) The right hand side of the linear system A*X = b. BNRM :IN Double Precision The scaled norm of b. X :IN Double Precision X(N) The current approximate solution as of the last restart. XL :IN Double Precision XL(N) An array of length N used to hold the approximate solution X(L) when ITOL=11. ITOL :IN Integer A flag to indicate the type of convergence criterion used. See the driver for its description. TOL :IN Double Precision The tolerance on residuals R0-A*Z in scaled norm. NELT :IN Integer The length of arrays IA, JA and A. IA :IN Integer IA(NELT) An integer array of length NELT containing matrix data. It is passed directly to the MATVEC and MSOLVE routines. JA :IN Integer JA(NELT) An integer array of length NELT containing matrix data. It is passed directly to the MATVEC and MSOLVE routines. A :IN Double Precision A(NELT) A double precision array of length NELT containing matrix data. It is passed directly to the MATVEC and MSOLVE routines. ISYM :IN Integer A flag to indicate symmetric matrix storage. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric and only the upper or lower triangular part is stored. IUNIT :IN Integer The i/o unit number for writing intermediate residual norm values. IFLAG :OUT Integer An integer error flag.. 0 means convergence in LGMR iterations, LGMR.le.MAXL. 1 means the convergence test did not pass in MAXL iterations, but the residual norm is .lt. norm(R0), and so Z is computed. 2 means the convergence test did not pass in MAXL iterations, residual .ge. norm(R0), and Z = 0. ERR :OUT Double Precision. Error estimate of error in final approximate solution, as defined by ITOL. *Cautions: This routine will attempt to write to the Fortran logical output unit IUNIT, if IUNIT .ne. 0. Thus, the user must make sure that this logical unit is attached to a file or terminal before calling this routine with a non-zero value for IUNIT. This routine does not check for the validity of a non-zero IUNIT unit number.

## Classification

- Historical role: `subsidiary`
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

- Canonical provider: `lin/dpigmr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dpigmr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dpigmr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
