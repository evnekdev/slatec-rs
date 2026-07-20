# SSLI2

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

SLAP Lower Triangle Matrix Backsolve. Routine to solve a system of the form Lx = b , where L is a lower triangular matrix.

## Description

*Usage: INTEGER N, NEL, IEL(NEL), JEL(NEL) REAL B(N), X(N), EL(NEL) CALL SSLI2( N, B, X, NEL, IEL, JEL, EL ) *Arguments: N :IN Integer Order of the Matrix. B :IN Real B(N). Right hand side vector. X :OUT Real X(N). Solution to Lx = b. NEL :IN Integer. Number of non-zeros in the EL array. IEL :IN Integer IEL(NEL). JEL :IN Integer JEL(NEL). EL :IN Real EL(NEL). IEL, JEL, EL contain the unit lower triangular factor of the incomplete decomposition of the A matrix stored in SLAP Row format. The diagonal of ones *IS* stored. This structure can be set up by the SS2LT routine. See the "Description", below, for more details about the SLAP Row format. *Description: This routine is supplied with the SLAP package as a routine to perform the MSOLVE operation in the SIR iteration routine for the driver routine SSGS. It must be called via the SLAP MSOLVE calling sequence convention interface routine SSLI. **** THIS ROUTINE ITSELF DOES NOT CONFORM TO THE **** **** SLAP MSOLVE CALLING CONVENTION **** ==================== S L A P Row format ==================== This routine requires that the matrix A be stored in the SLAP Row format. In this format the non-zeros are stored counting across rows (except for the diagonal entry, which must appear first in each "row") and are stored in the real array A. In other words, for each row in the matrix put the diagonal entry in A. Then put in the other non-zero elements going across the row (except the diagonal) in order. The JA array holds the column index for each non-zero. The IA array holds the offsets into the JA, A arrays for the beginning of each row. That is, JA(IA(IROW)), A(IA(IROW)) points to the beginning of the IROW-th row in JA and A. JA(IA(IROW+1)-1), A(IA(IROW+1)-1) points to the end of the IROW-th row. Note that we always have IA(N+1) = NELT+1, where N is the number of rows in the matrix and NELT is the number of non-zeros in the matrix. Here is an example of the SLAP Row storage format for a 5x5 Matrix (in the A and JA arrays '|' denotes the end of a row): 5x5 Matrix SLAP Row format for 5x5 matrix on left. 1 2 3 4 5 6 7 8 9 10 11 |11 12 0 0 15| A: 11 12 15 | 22 21 | 33 35 | 44 | 55 51 53 |21 22 0 0 0| JA: 1 2 5 | 2 1 | 3 5 | 4 | 5 1 3 | 0 0 33 0 35| IA: 1 4 6 8 9 12 | 0 0 0 44 0| |51 0 53 0 55| With the SLAP Row format the "inner loop" of this routine should vectorize on machines with hardware support for vector gather/scatter operations. Your compiler may require a compiler directive to convince it that there are no implicit vector dependencies. Compiler directives for the Alliant FX/Fortran and CRI CFT/CFT77 compilers are supplied with the standard SLAP distribution.

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
- GAMS classifications: `D2A3`
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

- Canonical provider: `lin/ssli2.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssli2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Dense linear algebra](../families/dense-linear-algebra.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, NEL, IEL(NEL), JEL(NEL) REAL B(N), X(N), EL(NEL) CALL SSLI2( N, B, X, NEL, IEL, JEL, EL ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (N) | *Usage: INTEGER N, NEL, IEL(NEL), JEL(NEL) REAL B(N), X(N), EL(NEL) CALL SSLI2( N, B, X, NEL, IEL, JEL, EL ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (N) | *Usage: INTEGER N, NEL, IEL(NEL), JEL(NEL) REAL B(N), X(N), EL(NEL) CALL SSLI2( N, B, X, NEL, IEL, JEL, EL ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEL` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, NEL, IEL(NEL), JEL(NEL) REAL B(N), X(N), EL(NEL) CALL SSLI2( N, B, X, NEL, IEL, JEL, EL ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IEL` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (NEL) | *Usage: INTEGER N, NEL, IEL(NEL), JEL(NEL) REAL B(N), X(N), EL(NEL) CALL SSLI2( N, B, X, NEL, IEL, JEL, EL ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JEL` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (NEL) | *Usage: INTEGER N, NEL, IEL(NEL), JEL(NEL) REAL B(N), X(N), EL(NEL) CALL SSLI2( N, B, X, NEL, IEL, JEL, EL ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EL` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (NEL) | *Usage: INTEGER N, NEL, IEL(NEL), JEL(NEL) REAL B(N), X(N), EL(NEL) CALL SSLI2( N, B, X, NEL, IEL, JEL, EL ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::dense::ssli2`. Native symbol: `ssli2_`. Feature: `linear-algebra`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::ssli2`
- Compatibility aliases: `none`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
