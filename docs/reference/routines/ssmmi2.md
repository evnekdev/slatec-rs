# SSMMI2

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

SLAP Backsolve for LDU Factorization of Normal Equations. To solve a system of the form (L*D*U)*(L*D*U)' X = B, where L is a unit lower triangular matrix, D is a diagonal matrix, and U is a unit upper triangular matrix and ' denotes transpose.

## Description

*Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) REAL B(N), X(N), L(NL), DINV(N), U(NU) CALL SSMMI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) *Arguments: N :IN Integer Order of the Matrix. B :IN Real B(N). Right hand side. X :OUT Real X(N). Solution of (L*D*U)(L*D*U)trans x = b. IL :IN Integer IL(NL). JL :IN Integer JL(NL). L :IN Real L(NL). IL, JL, L contain the unit lower triangular factor of the incomplete decomposition of some matrix stored in SLAP Row format. The diagonal of ones *IS* stored. This structure can be set up by the SSILUS routine. See the "Description", below for more details about the SLAP format. (NL is the number of non-zeros in the L array.) DINV :IN Real DINV(N). Inverse of the diagonal matrix D. IU :IN Integer IU(NU). JU :IN Integer JU(NU). U :IN Real U(NU). IU, JU, U contain the unit upper triangular factor of the incomplete decomposition of some matrix stored in SLAP Column format. The diagonal of ones *IS* stored. This structure can be set up by the SSILUS routine. See the "Description", below for more details about the SLAP format. (NU is the number of non-zeros in the U array.) *Description: This routine is supplied with the SLAP package as a routine to perform the MSOLVE operation in the SBCGN iteration routine for the driver SSLUCN. It must be called via the SLAP MSOLVE calling sequence convention interface routine SSMMTI. **** THIS ROUTINE ITSELF DOES NOT CONFORM TO THE **** **** SLAP MSOLVE CALLING CONVENTION **** IL, JL, L should contain the unit lower triangular factor of the incomplete decomposition of the A matrix stored in SLAP Row format. IU, JU, U should contain the unit upper factor of the incomplete decomposition of the A matrix stored in SLAP Column format This ILU factorization can be computed by the SSILUS routine. The diagonals (which are all one's) are stored. =================== S L A P Column format ================== This routine requires that the matrix A be stored in the SLAP Column format. In this format the non-zeros are stored counting down columns (except for the diagonal entry, which must appear first in each "column") and are stored in the real array A. In other words, for each column in the matrix put the diagonal entry in A. Then put in the other non-zero elements going down the column (except the diagonal) in order. The IA array holds the row index for each non-zero. The JA array holds the offsets into the IA, A arrays for the beginning of each column. That is, IA(JA(ICOL)), A(JA(ICOL)) points to the beginning of the ICOL-th column in IA and A. IA(JA(ICOL+1)-1), A(JA(ICOL+1)-1) points to the

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

- Canonical provider: `lin/ssmmi2.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssmmi2.f) — `verified_cached`
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
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) REAL B(N), X(N), L(NL), DINV(N), U(NU) CALL SSMMI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (N) | *Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) REAL B(N), X(N), L(NL), DINV(N), U(NU) CALL SSMMI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (N) | *Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) REAL B(N), X(N), L(NL), DINV(N), U(NU) CALL SSMMI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IL` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | *Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) REAL B(N), X(N), L(NL), DINV(N), U(NU) CALL SSMMI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JL` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | *Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) REAL B(N), X(N), L(NL), DINV(N), U(NU) CALL SSMMI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `L` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | *Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) REAL B(N), X(N), L(NL), DINV(N), U(NU) CALL SSMMI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `DINV` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (N) | *Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) REAL B(N), X(N), L(NL), DINV(N), U(NU) CALL SSMMI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IU` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | *Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) REAL B(N), X(N), L(NL), DINV(N), U(NU) CALL SSMMI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JU` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | *Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) REAL B(N), X(N), L(NL), DINV(N), U(NU) CALL SSMMI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `U` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (N) | *Usage: INTEGER N, IL(NL), JL(NL), IU(NU), JU(NU) REAL B(N), X(N), L(NL), DINV(N), U(NU) CALL SSMMI2( N, B, X, IL, JL, L, DINV, IU, JU, U ) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::dense::ssmmi2`. Native symbol: `ssmmi2_`. Feature: `linear-algebra`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::ssmmi2`
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
