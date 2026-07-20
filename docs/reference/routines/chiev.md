# CHIEV

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and, optionally, the eigenvectors of a complex Hermitian matrix.

## Description

David Kahaner, Cleve Moler, G. W. Stewart, N.B.S. U.N.M. N.B.S./U.MD. Abstract CHIEV computes the eigenvalues and, optionally, the eigenvectors of a complex Hermitian matrix. Call Sequence Parameters(the values of parameters marked with * (star) will be changed by CHIEV.) A* COMPLEX(LDA,N) complex Hermitian input matrix. Only the upper triangle of A need be filled in. Elements on diagonal must be real. LDA INTEGER set by the user to the leading dimension of the complex array A. N INTEGER set by the user to the order of the matrices A and V, and the number of elements in E. E* REAL(N) on return from CHIEV E contains the eigenvalues of A. See also INFO below. V* COMPLEX(LDV,N) on return from CHIEV if the user has set JOB = 0 V is not referenced. = nonzero the N eigenvectors of A are stored in the first N columns of V. See also INFO below. LDV INTEGER set by the user to the leading dimension of the array V if JOB is also set nonzero. In that case N must be .LE. LDV. If JOB is set to zero LDV is not referenced. WORK* REAL(4N) temporary storage vector. Contents changed by CHIEV. JOB INTEGER set by the user to = 0 eigenvalues only to be calculated by CHIEV. Neither V nor LDV are referenced. = nonzero eigenvalues and vectors to be calculated. In this case A and V must be distinct arrays also if LDA .GT. LDV CHIEV changes all the elements of A thru column N. If LDA < LDV CHIEV changes all the elements of V through column N. If LDA = LDV only A(I,J) and V(I, J) for I,J = 1,...,N are changed by CHIEV. INFO* INTEGER on return from CHIEV the value of INFO is = 0 normal return, calculation successful. = K if the eigenvalue iteration fails to converge, eigenvalues (and eigenvectors if requested) 1 through K-1 are correct. Error Messages No. 1 recoverable N is greater than LDA No. 2 recoverable N is less than one. No. 3 recoverable JOB is nonzero and N is greater than LDV No. 4 warning LDA > LDV, elements of A other than the N by N input elements have been changed No. 5 warning LDA < LDV, elements of V other than the N by N output elements have been changed No. 6 recoverable nonreal element on diagonal of A.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4A3`
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

- Canonical provider: `main-src/src/chiev.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/chiev.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/chiev.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/chiev.f) — `verified_cached`
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
| `A` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | Abstract CHIEV computes the eigenvalues and, optionally, the eigenvectors of a complex Hermitian matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDA` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Call Sequence Parameters(the values of parameters marked with * (star) will be changed by CHIEV.) A* COMPLEX(LDA,N) complex Hermitian input matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Stewart, N.B.S. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `E` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | N INTEGER set by the user to the order of the matrices A and V, and the number of elements in E. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `V` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | N INTEGER set by the user to the order of the matrices A and V, and the number of elements in E. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDV` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | V* COMPLEX(LDV,N) on return from CHIEV if the user has set JOB = 0 V is not referenced. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | WORK* REAL(4N) temporary storage vector. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JOB` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | V* COMPLEX(LDV,N) on return from CHIEV if the user has set JOB = 0 V is not referenced. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | See also INFO below. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::eigen::chiev`. Native symbol: `chiev_`. Feature: `eigen`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::chiev`
- Compatibility aliases: `slatec_sys::eigen::numerical::chiev`
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
