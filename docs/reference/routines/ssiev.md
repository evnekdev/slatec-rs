# SSIEV

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix.

## Description

Abstract SSIEV computes the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix. Call Sequence Parameters(The values of parameters marked with * (star) will be changed by SSIEV.) A* REAL (LDA,N) real symmetric input matrix. Only the diagonal and upper triangle of A must be input, as SSIEV copies the upper triangle to the lower. That is, the user must define A(I,J), I=1,..N, and J=I,. ..,N. On return from SSIEV, if the user has set JOB = 0 the lower triangle of A has been altered. = nonzero the N eigenvectors of A are stored in its first N columns. See also INFO below. LDA INTEGER set by the user to the leading dimension of the array A. N INTEGER set by the user to the order of the matrix A and the number of elements in E. E* REAL (N) on return from SSIEV, E contains the N eigenvalues of A. See also INFO below. WORK* REAL (2*N) temporary storage vector. Contents changed by SSIEV. JOB INTEGER set by user on input = 0 only calculate eigenvalues of A. = nonzero calculate eigenvalues and eigenvectors of A. INFO* INTEGER on return from SSIEV, the value of INFO is = 0 for normal return. = K if the eigenvalue iteration fails to converge. eigenvalues and vectors 1 through K-1 are correct. Error MessagesNo. 1 recoverable N is greater than LDA No. 2 recoverable N is less than one

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4A1`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/ssiev.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ssiev.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ssiev.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ssiev.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [Eigenvalue problems](../families/eigenvalue-problems.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `A` | output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDA, *) | Abstract SSIEV computes the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDA` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Call Sequence Parameters(The values of parameters marked with * (star) will be changed by SSIEV.) A* REAL (LDA,N) real symmetric input matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Call Sequence Parameters(The values of parameters marked with * (star) will be changed by SSIEV.) A* REAL (LDA,N) real symmetric input matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `E` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | N INTEGER set by the user to the order of the matrix A and the number of elements in E. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | WORK* REAL (2*N) temporary storage vector. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JOB` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | On return from SSIEV, if the user has set JOB = 0 the lower triangle of A has been altered. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | See also INFO below. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::eigen::ssiev`. Native symbol: `ssiev_`. Feature: `eigen`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::ssiev`
- Compatibility aliases: `slatec_sys::eigen::numerical::ssiev`
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
