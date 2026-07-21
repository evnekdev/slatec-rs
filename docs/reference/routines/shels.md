# SHELS

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Internal routine for SGMRES.

## Description

This routine is extracted from the LINPACK routine SGESL with changes due to the fact that A is an upper Hessenberg matrix. SHELS solves the least squares problem: MIN(B-A*X,B-A*X) using the factors computed by SHEQR. *Usage: INTEGER LDA, N REAL A(LDA,N), Q(2*N), B(N+1) CALL SHELS(A, LDA, N, Q, B) *Arguments: A :IN Real A(LDA,N) The output from SHEQR which contains the upper triangular factor R in the QR decomposition of A. LDA :IN Integer The leading dimension of the array A. N :IN Integer A is originally an (N+1) by N matrix. Q :IN Real Q(2*N) The coefficients of the N Givens rotations used in the QR factorization of A. B :INOUT Real B(N+1) On input, B is the right hand side vector. On output, B is the solution vector X.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
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

- Canonical provider: `lin/shels.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/shels.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/shels.f) — `verified_cached`
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
| `A` | output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDA, *) | This routine is extracted from the LINPACK routine SGESL with changes due to the fact that A is an upper Hessenberg matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDA` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER LDA, N REAL A(LDA,N), Q(2*N), B(N+1) CALL SHELS(A, LDA, N, Q, B) *Arguments: A :IN Real A(LDA,N) The output from SHEQR which contains the upper triangular factor R in the QR decomposition of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: INTEGER LDA, N REAL A(LDA,N), Q(2*N), B(N+1) CALL SHELS(A, LDA, N, Q, B) *Arguments: A :IN Real A(LDA,N) The output from SHEQR which contains the upper triangular factor R in the QR decomposition of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Q` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | *Usage: INTEGER LDA, N REAL A(LDA,N), Q(2*N), B(N+1) CALL SHELS(A, LDA, N, Q, B) *Arguments: A :IN Real A(LDA,N) The output from SHEQR which contains the upper triangular factor R in the QR decomposition of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | SHELS solves the least squares problem: MIN(B-A*X,B-A*X) using the factors computed by SHEQR. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
