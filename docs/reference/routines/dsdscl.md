# DSDSCL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Diagonal Scaling of system Ax = b. This routine scales (and unscales) the system Ax = b by symmetric diagonal scaling.

## Description

This routine scales (and unscales) the system Ax = b by symmetric diagonal scaling. The new system is: -1/2 -1/2 1/2 -1/2 D AD (D x) = D b when scaling is selected with the JOB parameter. When unscaling is selected this process is reversed. The true solution is also scaled or unscaled if ITOL is set appropriately, see below. *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, JOB, ITOL DOUBLE PRECISION A(NELT), X(N), B(N), DINV(N) CALL DSDSCL( N, NELT, IA, JA, A, ISYM, X, B, DINV, JOB, ITOL ) *Arguments: N :IN Integer Order of the Matrix. NELT :IN Integer. Number of elements in arrays IA, JA, and A. IA :IN Integer IA(NELT). JA :IN Integer JA(NELT). A :IN Double Precision A(NELT). These arrays should hold the matrix A in the SLAP Column format. See "Description", below. ISYM :IN Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the upper or lower triangle of the matrix is stored. X :INOUT Double Precision X(N). Initial guess that will be later used in the iterative solution. of the scaled system. B :INOUT Double Precision B(N). Right hand side vector. DINV :INOUT Double Precision DINV(N). Upon return this array holds 1./DIAG(A). This is an input if JOB = 0. JOB :IN Integer. Flag indicating whether to scale or not. JOB non-zero means do scaling. JOB = 0 means do unscaling. ITOL :IN Integer. Flag indicating what type of error estimation to do in the iterative method. When ITOL = 11 the exact solution from common block DSLBLK will be used. When the system is scaled then the true solution must also be scaled. If ITOL is not 11 then this vector is not referenced. *Common Blocks: SOLN :INOUT Double Precision SOLN(N). COMMON BLOCK /DSLBLK/ The true solution, SOLN, is scaled (or unscaled) if ITOL is set to 11, see above. *Description =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the double precision array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the end of the ICOL-th column. Note that we always have JA(N+1) = NELT+1, where N is the number of columns in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Column storage format for a 5x5 Matrix (in the A and IA arrays '|' denotes the end of a column): 5x5 Matrix SLAP Column format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 21 51 | 22 12 | 33 53 | 44 | 55 15 35 |21 22 0 0 0| IA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| JA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| With the SLAP format all of the "inner loops" of this routine should vectorize on machines with hardware support for vector gather/scatter operations. Your compiler may require a compiler directive to convince it that there are no implicit vector dependencies. Compiler directives for the Alliant FX/Fortran and CRI CFT/CFT77 compilers are supplied with the standard SLAP distribution. *Cautions: This routine assumes that the diagonal of A is all non-zero and that the operation DINV = 1.0/DIAG(A) will not underflow or overflow. This is done so that the loop vectorizes. Matrices with zero or near zero or very large entries will have numerical difficulties and must be fixed before this routine is called.

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
- GAMS classifications: `D2E`
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

- Canonical provider: `lin/dsdscl.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dsdscl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
