# SBHIN

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Read a Sparse Linear System in the Boeing/Harwell Format. The matrix is read in and if the right hand side is also present in the input file then it too is read in. The matrix is then modified to be in the SLAP Column format.

## Description

*Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IUNIT, JOB REAL A(NELT), SOLN(N), RHS(N) CALL SBHIN( N, NELT, IA, JA, A, ISYM, SOLN, RHS, IUNIT, JOB ) *Arguments: N :OUT Integer Order of the Matrix. NELT :INOUT Integer. On input NELT is the maximum number of non-zeros that can be stored in the IA, JA, A arrays. On output NELT is the number of non-zeros stored in A. IA :OUT Integer IA(NELT). JA :OUT Integer JA(NELT). A :OUT Real A(NELT). On output these arrays hold the matrix A in the SLAP Triad format. See "Description", below. ISYM :OUT Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the lower triangle of the matrix is stored. SOLN :OUT Real SOLN(N). The solution to the linear system, if present. This array is accessed if and only if JOB is set to read it in, see below. If the user requests that SOLN be read in, but it is not in the file, then it is simply zeroed out. RHS :OUT Real RHS(N). The right hand side vector. This array is accessed if and only if JOB is set to read it in, see below. If the user requests that RHS be read in, but it is not in the file, then it is simply zeroed out. IUNIT :IN Integer. Fortran logical I/O device unit number to read the matrix from. This unit must be connected in a system dependent fashion to a file, or you will get a nasty message from the Fortran I/O libraries. JOB :INOUT Integer. Flag indicating what I/O operations to perform. On input JOB indicates what Input operations to try to perform. JOB = 0 => Read only the matrix. JOB = 1 => Read matrix and RHS (if present). JOB = 2 => Read matrix and SOLN (if present). JOB = 3 => Read matrix, RHS and SOLN (if present). On output JOB indicates what operations were actually performed. JOB = -3 => Unable to parse matrix "CODE" from input file to determine if only the lower triangle of matrix is stored. JOB = -2 => Number of non-zeros (NELT) too large. JOB = -1 => System size (N) too large. JOB = 0 => Read in only the matrix. JOB = 1 => Read in the matrix and RHS. JOB = 2 => Read in the matrix and SOLN. JOB = 3 => Read in the matrix, RHS and SOLN. JOB = 10 => Read in only the matrix *STRUCTURE*, but no non-zero entries. Hence, A(*) is not referenced and has the return values the same as the input. JOB = 11 => Read in the matrix *STRUCTURE* and RHS. JOB = 12 => Read in the matrix *STRUCTURE* and SOLN. JOB = 13 => Read in the matrix *STRUCTURE*, RHS and SOLN. *Description: The format for the input is as follows. The first line contains a title to identify the data file. On the second line (5I4) are counters: NLINE, NPLS, NRILS, NNVLS, NRHSLS. NLINE Number of data lines (after the header) in the file. NPLS Number of lines for the Column Pointer data in the file. NRILS Number of lines for the Row indices in the file. NNVLS Number of lines for the Matrix elements in the file. NRHSLS Number of lines for the RHS in the file. The third line (A3,11X,4I4) contains a symmetry code and some additional counters: CODE, NROW, NCOL, NIND, NELE. On the fourth line (2A16,2A20) are formats to be used to read the following data: PNTFNT, RINFMT, NVLFMT, RHSFMT. Following that are the blocks of data in the order indicated. =================== S L A P Triad format =================== This routine requires that the matrix A be stored in the SLAP Triad format. In this format only the non-zeros are stored. They may appear in *ANY* order. The user supplies three arrays of length NELT, where NELT is the number of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)). For each non-zero the user puts the row and column index of that matrix element in the IA and JA arrays. The value of the non-zero matrix element is placed in the corresponding location of the A array. This is an extremely easy data structure to generate. On the other hand it is not too efficient on vector computers for the iterative solution of linear systems. Hence, SLAP changes this input data structure to the SLAP Column format for the iteration (but does not change it back). Here is an example of the SLAP Triad storage format for a 5x5 Matrix. Recall that the entries may appear in any order. 5x5 Matrix SLAP Triad format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 51 12 11 33 15 53 55 22 35 44 21 |21 22 0 0 0| IA: 5 1 1 3 1 5 5 2 3 4 2 | 0 0 33 0 35| JA: 1 2 1 3 5 3 5 2 5 4 1 | 0 0 0 44 0| |51 0 53 0 55| *Portability: You must make sure that IUNIT is a valid Fortran logical I/O device unit number and that the unit number has been associated with a file or the console. This is a system dependent function. *Implementation note: SOLN is not read by this version. It will simply be zeroed out if JOB = 2 or 3 and the returned value of JOB will indicate SOLN has not been read.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Shared numerical utilities`
- Mathematical domain: `data-utilities`
- Package provenance: `unknown`
- GAMS classifications: `N1`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/sbhin.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sbhin.f) — `verified_cached`
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
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
