# BALANC

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Balance a real general matrix and isolate eigenvalues whenever possible.

## Description

This subroutine is a translation of the ALGOL procedure BALANCE, NUM. MATH. 13, 293-304(1969) by Parlett and Reinsch. HANDBOOK FOR AUTO. COMP., Vol.II-LINEAR ALGEBRA, 315-326(1971). This subroutine balances a REAL matrix and isolates eigenvalues whenever possible. On INPUT NM must be set to the row dimension of the two-dimensional array parameter, A, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix A. N is an INTEGER variable. N must be less than or equal to NM. A contains the input matrix to be balanced. A is a two-dimensional REAL array, dimensioned A(NM,N). On OUTPUT A contains the balanced matrix. LOW and IGH are two INTEGER variables such that A(I,J) is equal to zero if (1) I is greater than J and (2) J=1,...,LOW-1 or I=IGH+1,...,N. SCALE contains information determining the permutations and scaling factors used. SCALE is a one-dimensional REAL array, dimensioned SCALE(N). Suppose that the principal submatrix in rows LOW through IGH has been balanced, that P(J) denotes the index interchanged with J during the permutation step, and that the elements of the diagonal matrix used are denoted by D(I,J). Then SCALE(J) = P(J), for J = 1,...,LOW-1 = D(J,J), J = LOW,...,IGH = P(J) J = IGH+1,...,N. The order in which the interchanges are made is N to IGH+1, then 1 TO LOW-1. Note that 1 is returned for IGH if IGH is zero formally. The ALGOL procedure EXC contained in BALANCE appears in BALANC in line. (Note that the ALGOL roles of identifiers K,L have been reversed.) Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4C1A`
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

- Canonical provider: `lin/balanc.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/balanc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Eigenvalue problems](../families/eigenvalue-problems.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `NM` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | On INPUT NM must be set to the row dimension of the two-dimensional array parameter, A, as declared in the calling program dimension statement. | On INPUT NM must be set to the row dimension of the two-dimensional array parameter, A, as declared in the calling program dimension statement. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | N is the order of the matrix A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (NM, *) | This subroutine is a translation of the ALGOL procedure BALANCE, NUM. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LOW` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | LOW and IGH are two INTEGER variables such that A(I,J) is equal to zero if (1) I is greater than J and (2) J=1,...,LOW-1 or I=IGH+1,...,N. | LOW and IGH are two INTEGER variables such that A(I,J) is equal to zero if (1) I is greater than J and (2) J=1,...,LOW-1 or I=IGH+1,...,N. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IGH` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | LOW and IGH are two INTEGER variables such that A(I,J) is equal to zero if (1) I is greater than J and (2) J=1,...,LOW-1 or I=IGH+1,...,N. | LOW and IGH are two INTEGER variables such that A(I,J) is equal to zero if (1) I is greater than J and (2) J=1,...,LOW-1 or I=IGH+1,...,N. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SCALE` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | SCALE contains information determining the permutations and scaling factors used. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::eigen::balanc`. Native symbol: `balanc_`. Feature: `eigen`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::balanc`
- Compatibility aliases: `slatec_sys::eigen::numerical::balanc`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
