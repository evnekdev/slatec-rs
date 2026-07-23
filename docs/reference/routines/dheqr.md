# DHEQR

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Internal routine for DGMRES.

## Description

This routine performs a QR decomposition of an upper Hessenberg matrix A using Givens rotations. There are two options available: 1) Performing a fresh decomposition 2) updating the QR factors by adding a row and a column to the matrix A. *Usage: INTEGER LDA, N, INFO, IJOB DOUBLE PRECISION A(LDA,N), Q(2*N) CALL DHEQR(A, LDA, N, Q, INFO, IJOB) *Arguments: A :INOUT Double Precision A(LDA,N) On input, the matrix to be decomposed. On output, the upper triangular matrix R. The factorization can be written Q*A = R, where Q is a product of Givens rotations and R is upper triangular. LDA :IN Integer The leading dimension of the array A. N :IN Integer A is an (N+1) by N Hessenberg matrix. Q :OUT Double Precision Q(2*N) The factors c and s of each Givens rotation used in decomposing A. INFO :OUT Integer = 0 normal value. = K if A(K,K) .eq. 0.0 . This is not an error condition for this subroutine, but it does indicate that DHELS will divide by zero if called. IJOB :IN Integer = 1 means that a fresh decomposition of the matrix A is desired. .ge. 2 means that the current decomposition of A will be updated by the addition of a row and a column.

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
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/dheqr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dheqr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Dense linear algebra](../families/dense-linear-algebra.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `A` | input/output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (LDA, *) | This routine performs a QR decomposition of an upper Hessenberg matrix A using Givens rotations. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDA` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER LDA, N, INFO, IJOB DOUBLE PRECISION A(LDA,N), Q(2*N) CALL DHEQR(A, LDA, N, Q, INFO, IJOB) *Arguments: A :INOUT Double Precision A(LDA,N) On input, the matrix to be decomposed. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER LDA, N, INFO, IJOB DOUBLE PRECISION A(LDA,N), Q(2*N) CALL DHEQR(A, LDA, N, Q, INFO, IJOB) *Arguments: A :INOUT Double Precision A(LDA,N) On input, the matrix to be decomposed. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Q` | input/output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | *Usage: INTEGER LDA, N, INFO, IJOB DOUBLE PRECISION A(LDA,N), Q(2*N) CALL DHEQR(A, LDA, N, Q, INFO, IJOB) *Arguments: A :INOUT Double Precision A(LDA,N) On input, the matrix to be decomposed. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER LDA, N, INFO, IJOB DOUBLE PRECISION A(LDA,N), Q(2*N) CALL DHEQR(A, LDA, N, Q, INFO, IJOB) *Arguments: A :INOUT Double Precision A(LDA,N) On input, the matrix to be decomposed. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IJOB` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER LDA, N, INFO, IJOB DOUBLE PRECISION A(LDA,N), Q(2*N) CALL DHEQR(A, LDA, N, Q, INFO, IJOB) *Arguments: A :INOUT Double Precision A(LDA,N) On input, the matrix to be decomposed. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
