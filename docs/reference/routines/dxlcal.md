# DXLCAL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Internal routine for DGMRES.

## Description

This routine computes the solution XL, the current DGMRES iterate, given the V(I)'s and the QR factorization of the Hessenberg matrix HES. This routine is only called when ITOL=11. *Usage: INTEGER N, LGMR, MAXLP1, JSCAL, JPRE, NMSL, IPAR(USER DEFINED) INTEGER NELT, IA(NELT), JA(NELT), ISYM DOUBLE PRECISION X(N), XL(N), ZL(N), HES(MAXLP1,MAXL), Q(2*MAXL), $ V(N,MAXLP1), R0NRM, WK(N), SZ(N), $ RPAR(USER DEFINED), A(NELT) EXTERNAL MSOLVE CALL DXLCAL(N, LGMR, X, XL, ZL, HES, MAXLP1, Q, V, R0NRM, $ WK, SZ, JSCAL, JPRE, MSOLVE, NMSL, RPAR, IPAR, $ NELT, IA, JA, A, ISYM) *Arguments: N :IN Integer The order of the matrix A, and the lengths of the vectors SR, SZ, R0 and Z. LGMR :IN Integer The number of iterations performed and the current order of the upper Hessenberg matrix HES. X :IN Double Precision X(N) The current approximate solution as of the last restart. XL :OUT Double Precision XL(N) An array of length N used to hold the approximate solution X(L). Warning: XL and ZL are the same array in the calling routine. ZL :IN Double Precision ZL(N) An array of length N used to hold the approximate solution Z(L). HES :IN Double Precision HES(MAXLP1,MAXL) The upper triangular factor of the QR decomposition of the (LGMR+1) by LGMR upper Hessenberg matrix whose entries are the scaled inner-products of A*V(*,i) and V(*,k). MAXLP1 :IN Integer MAXLP1 = MAXL + 1, used for dynamic dimensioning of HES. MAXL is the maximum allowable order of the matrix HES. Q :IN Double Precision Q(2*MAXL) A double precision array of length 2*MAXL containing the components of the Givens rotations used in the QR decomposition of HES. It is loaded in DHEQR. V :IN Double Precision V(N,MAXLP1) The N by(LGMR+1) array containing the LGMR orthogonal vectors V(*,1) to V(*,LGMR). R0NRM :IN Double Precision The scaled norm of the initial residual for the current call to DPIGMR. WK :IN Double Precision WK(N) A double precision work array of length N. SZ :IN Double Precision SZ(N) A vector of length N containing the non-zero elements of the diagonal scaling matrix for Z. JSCAL :IN Integer A flag indicating whether arrays SR and SZ are used. JSCAL=0 means SR and SZ are not used and the algorithm will perform as if all SR(i) = 1 and SZ(i) = 1. JSCAL=1 means only SZ is used, and the algorithm performs as if all SR(i) = 1. JSCAL=2 means only SR is used, and the algorithm performs as if all SZ(i) = 1. JSCAL=3 means both SR and SZ are used. JPRE :IN Integer The preconditioner type flag. MSOLVE :EXT External. Name of the routine which solves a linear system Mz = r for z given r with the preconditioning matrix M (M is supplied via RPAR and IPAR arrays. The name of the MSOLVE routine must be declared external in the calling program. The calling sequence to MSOLVE is: CALL MSOLVE(N, R, Z, NELT, IA, JA, A, ISYM, RPAR, IPAR) Where N is the number of unknowns, R is the right-hand side vector and Z is the solution upon return. NELT, IA, JA, A and ISYM are defined as below. RPAR is a double precision array that can be used to pass necessary preconditioning information and/or workspace to MSOLVE. IPAR is an integer work array for the same purpose as RPAR. NMSL :IN Integer The number of calls to MSOLVE. RPAR :IN Double Precision RPAR(USER DEFINED) Double Precision workspace passed directly to the MSOLVE routine. IPAR :IN Integer IPAR(USER DEFINED) Integer workspace passed directly to the MSOLVE routine. NELT :IN Integer The length of arrays IA, JA and A. IA :IN Integer IA(NELT) An integer array of length NELT containing matrix data. It is passed directly to the MATVEC and MSOLVE routines. JA :IN Integer JA(NELT) An integer array of length NELT containing matrix data. It is passed directly to the MATVEC and MSOLVE routines. A :IN Double Precision A(NELT) A double precision array of length NELT containing matrix data. It is passed directly to the MATVEC and MSOLVE routines. ISYM :IN Integer A flag to indicate symmetric matrix storage. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric and only the upper or lower triangular part is stored.

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

- Canonical provider: `lin/dxlcal.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dxlcal.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dxlcal.f) — `verified_cached`
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
- Public declaration feature: `raw-ffi-callbacks`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
