# SSPEV

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix stored in packed form.

## Description

Abstract SSPEV computes the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix stored in packed form. Call Sequence Parameters(The values of parameters marked with * (star) will be changed by SSPEV.) A* REAL(N*(N+1)/2) real symmetric packed input matrix. Contains upper triangle and diagonal of A, by column (elements 11, 12, 22, 13, 23, 33, ...). N INTEGER set by the user to the order of the matrix A. E* REAL(N) on return from SSPEV, E contains the eigenvalues of A. See also INFO below. V* REAL(LDV,N) on return from SSPEV, if the user has set JOB = 0 V is not referenced. = nonzero the N eigenvectors of A are stored in the first N columns of V. See also INFO below. LDV INTEGER set by the user to the leading dimension of the array V if JOB is also set nonzero. In that case, N must be .LE. LDV. If JOB is set to zero, LDV is not referenced. WORK* REAL(2N) temporary storage vector. Contents changed by SSPEV. JOB INTEGER set by the user to = 0 eigenvalues only to be calculated by SSPEV. Neither V nor LDV are referenced. = nonzero eigenvalues and vectors to be calculated. In this case, A & V must be distinct arrays. Also, if LDA .GT. LDV, SSPEV changes all the elements of A thru column N. If LDA < LDV, SSPEV changes all the elements of V through column N. If LDA=LDV, only A(I,J) and V(I, J) for I,J = 1,...,N are changed by SSPEV. INFO* INTEGER on return from SSPEV, the value of INFO is = 0 for normal return. = K if the eigenvalue iteration fails to converge. Eigenvalues and vectors 1 through K-1 are correct. Error MessagesNo. 1 recoverable N is greater than LDV and JOB is nonzero No. 2 recoverable N is less than one

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

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/sspev.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sspev.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sspev.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
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
| `A` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | Abstract SSPEV computes the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix stored in packed form. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Call Sequence Parameters(The values of parameters marked with * (star) will be changed by SSPEV.) A* REAL(N*(N+1)/2) real symmetric packed input matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `E` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | E* REAL(N) on return from SSPEV, E contains the eigenvalues of A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `V` | output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDV, *) | V* REAL(LDV,N) on return from SSPEV, if the user has set JOB = 0 V is not referenced. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDV` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | V* REAL(LDV,N) on return from SSPEV, if the user has set JOB = 0 V is not referenced. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | WORK* REAL(2N) temporary storage vector. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JOB` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | V* REAL(LDV,N) on return from SSPEV, if the user has set JOB = 0 V is not referenced. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | See also INFO below. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::eigen::sspev`. Native symbol: `sspev_`. Feature: `eigen`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::sspev`
- Compatibility aliases: `slatec_sys::eigen::numerical::sspev`
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
