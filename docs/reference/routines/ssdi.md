# SSDI

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Diagonal Matrix Vector Multiply. Routine to calculate the product X = DIAG*B, where DIAG is a diagonal matrix.

## Description

*Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) *Arguments: N :IN Integer Order of the Matrix. B :IN Real B(N). Vector to multiply the diagonal by. X :OUT Real X(N). Result of DIAG*B. NELT :DUMMY Integer. IA :DUMMY Integer IA(NELT). JA :DUMMY Integer JA(NELT). A :DUMMY Real A(NELT). ISYM :DUMMY Integer. These are for compatibility with SLAP MSOLVE calling sequence. RWORK :IN Real RWORK(USER DEFINED). Work array holding the diagonal of some matrix to scale B by. This array must be set by the user or by a call to the SLAP routine SSDS or SSD2S. The length of RWORK must be >= IWORK(4)+N. IWORK :IN Integer IWORK(10). IWORK(4) holds the offset into RWORK for the diagonal matrix to scale B by. This is usually set up by the SLAP preconditioner setup routines SSDS or SSD2S. *Description: This routine is supplied with the SLAP package to perform the MSOLVE operation for iterative drivers that require diagonal Scaling (e.g., SSDCG, SSDBCG). It conforms to the SLAP MSOLVE CALLING CONVENTION and hence does not require an interface routine as do some of the other preconditioners supplied with SLAP.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- GAMS classifications: `D1B4`
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

- Canonical provider: `lin/ssdi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssdi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Linear algebra kernels](../families/linear-algebra-kernels.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (N) | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (N) | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NELT` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IA` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JA` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (NELT) | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (NELT) | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ISYM` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RWORK` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IWORK` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (10) | *Usage: INTEGER N, NELT, IA(NELT), JA(NELT), ISYM, IWORK(10) REAL B(N), X(N), A(NELT), RWORK(USER DEFINED) CALL SSDI (N, B, X, NELT, IA, JA, A, ISYM, RWORK, IWORK) *Arguments: N :IN Integer Order of the Matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::blas::level1::ssdi`. Native symbol: `ssdi_`. Feature: `blas-level1`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level1::ssdi`
- Compatibility aliases: `slatec_sys::families::blas_level1::ssdi`
- Public declaration feature: `blas-level1`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
