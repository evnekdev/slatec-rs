# STIN

[Family: Shared numerical utilities](../families/shared-numerical-utilities.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Read in SLAP Triad Format Linear System. Routine to read in a SLAP Triad format matrix and right hand side and solution to the system, if known.

## Description

*Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IUNIT, JOB REAL A(NELT), SOLN(N), RHS(N) CALL STIN( N, NELT, IA, JA, A, ISYM, SOLN, RHS, IUNIT, JOB ) *Arguments: N :OUT Integer Order of the Matrix. NELT :INOUT Integer. On input NELT is the maximum number of non-zeros that can be stored in the IA, JA, A arrays. On output NELT is the number of non-zeros stored in A. IA :OUT Integer IA(NELT). JA :OUT Integer JA(NELT). A :OUT Real A(NELT). On output these arrays hold the matrix A in the SLAP Triad format. See "Description", below. ISYM :OUT Integer. Flag to indicate symmetric storage format. If ISYM=0, all non-zero entries of the matrix are stored. If ISYM=1, the matrix is symmetric, and only the lower triangle of the matrix is stored. SOLN :OUT Real SOLN(N). The solution to the linear system, if present. This array is accessed if and only if JOB to read it in, see below. If the user requests that SOLN be read in, but it is not in the file, then it is simply zeroed out. RHS :OUT Real RHS(N). The right hand side vector. This array is accessed if and only if JOB is set to read it in, see below. If the user requests that RHS be read in, but it is not in the file, then it is simply zeroed out. IUNIT :IN Integer. Fortran logical I/O device unit number to write the matrix to. This unit must be connected in a system dependent fashion to a file or the console or you will get a nasty message from the Fortran I/O libraries. JOB :INOUT Integer. Flag indicating what I/O operations to perform. On input JOB indicates what Input operations to try to perform. JOB = 0 => Read only the matrix. JOB = 1 => Read matrix and RHS (if present). JOB = 2 => Read matrix and SOLN (if present). JOB = 3 => Read matrix, RHS and SOLN (if present). On output JOB indicates what operations were actually performed. JOB = 0 => Read in only the matrix. JOB = 1 => Read in the matrix and RHS. JOB = 2 => Read in the matrix and SOLN. JOB = 3 => Read in the matrix, RHS and SOLN. *Description: The format for the input is as follows. On the first line are counters and flags: N, NELT, ISYM, IRHS, ISOLN. N, NELT and ISYM are described above. IRHS is a flag indicating if the RHS was written out (1 is yes, 0 is no). ISOLN is a flag indicating if the SOLN was written out (1 is yes, 0 is no). The format for the fist line is: 5i10. Then comes the NELT Triad's IA(I), JA(I) and A(I), I = 1, NELT. The format for these lines is : 1X,I5,1X,I5,1X,E16.7. Then comes RHS(I), I = 1, N, if IRHS = 1. Then comes SOLN(I), I = 1, N, if ISOLN = 1. The format for these lines is: 1X,E16.7. =================== S L A P Triad format =================== This routine requires that the matrix A be stored in the SLAP Triad format. In this format only the non-zeros are stored. They may appear in *ANY* order. The user supplies three arrays of length NELT, where NELT is the number of non-zeros in the matrix: (IA(NELT), JA(NELT), A(NELT)). For each non-zero the user puts the row and column index of that matrix element in the IA and JA arrays. The value of the non-zero matrix element is placed in the corresponding location of the A array. This is an extremely easy data structure to generate. On the other hand it is not too efficient on vector computers for the iterative solution of linear systems. Hence, SLAP changes this input data structure to the SLAP Column format for the iteration (but does not change it back). Here is an example of the SLAP Triad storage format for a 5x5 Matrix. Recall that the entries may appear in any order. 5x5 Matrix SLAP Triad format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 51 12 11 33 15 53 55 22 35 44 21 |21 22 0 0 0| IA: 5 1 1 3 1 5 5 2 3 4 2 | 0 0 33 0 35| JA: 1 2 1 3 5 3 5 2 5 4 1 | 0 0 0 44 0| |51 0 53 0 55| *Cautions: This routine will attempt to write to the Fortran logical output unit IUNIT, if IUNIT .ne. 0. Thus, the user must make sure that this logical unit is attached to a file or terminal before calling this routine with a non-zero value for IUNIT. This routine does not check for the validity of a non-zero IUNIT unit number.

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

- Canonical provider: `lin/stin.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/stin.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Shared numerical utilities](../families/shared-numerical-utilities.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IUNIT, JOB REAL A(NELT), SOLN(N), RHS(N) CALL STIN( N, NELT, IA, JA, A, ISYM, SOLN, RHS, IUNIT, JOB ) *Arguments: N :OUT Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NELT` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IUNIT, JOB REAL A(NELT), SOLN(N), RHS(N) CALL STIN( N, NELT, IA, JA, A, ISYM, SOLN, RHS, IUNIT, JOB ) *Arguments: N :OUT Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IA` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IUNIT, JOB REAL A(NELT), SOLN(N), RHS(N) CALL STIN( N, NELT, IA, JA, A, ISYM, SOLN, RHS, IUNIT, JOB ) *Arguments: N :OUT Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JA` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IUNIT, JOB REAL A(NELT), SOLN(N), RHS(N) CALL STIN( N, NELT, IA, JA, A, ISYM, SOLN, RHS, IUNIT, JOB ) *Arguments: N :OUT Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (NELT) | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IUNIT, JOB REAL A(NELT), SOLN(N), RHS(N) CALL STIN( N, NELT, IA, JA, A, ISYM, SOLN, RHS, IUNIT, JOB ) *Arguments: N :OUT Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ISYM` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IUNIT, JOB REAL A(NELT), SOLN(N), RHS(N) CALL STIN( N, NELT, IA, JA, A, ISYM, SOLN, RHS, IUNIT, JOB ) *Arguments: N :OUT Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SOLN` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (N) | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IUNIT, JOB REAL A(NELT), SOLN(N), RHS(N) CALL STIN( N, NELT, IA, JA, A, ISYM, SOLN, RHS, IUNIT, JOB ) *Arguments: N :OUT Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RHS` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (N) | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IUNIT, JOB REAL A(NELT), SOLN(N), RHS(N) CALL STIN( N, NELT, IA, JA, A, ISYM, SOLN, RHS, IUNIT, JOB ) *Arguments: N :OUT Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IUNIT` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IUNIT, JOB REAL A(NELT), SOLN(N), RHS(N) CALL STIN( N, NELT, IA, JA, A, ISYM, SOLN, RHS, IUNIT, JOB ) *Arguments: N :OUT Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JOB` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IUNIT, JOB REAL A(NELT), SOLN(N), RHS(N) CALL STIN( N, NELT, IA, JA, A, ISYM, SOLN, RHS, IUNIT, JOB ) *Arguments: N :OUT Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Compatibility aliases: `none`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
