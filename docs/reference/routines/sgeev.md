# SGEEV

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and, optionally, the eigenvectors of a real general matrix.

## Description

SGEEV computes the eigenvalues and, optionally, the eigenvectors of a general real matrix. Call Sequence Parameters- (The values of parameters marked with * (star) will be changed by SGEEV.)

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
- GAMS classifications: `D4A2`
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

- Canonical provider: `main-src/src/sgeev.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sgeev.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sgeev.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sgeev.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [SGEEV](https://www.netlib.org/slatec/src/sgeev.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(LDA,N) real nonsymmetric input matrix. |
| 2 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER set by the user to the leading dimension of the real array A. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER set by the user to the order of the matrices A and V, and the number of elements in E. |
| 4 | `E` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | COMPLEX(N) on return from SGEEV, E contains the eigenvalues of A. See also INFO below. |
| 5 | `V` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | COMPLEX(LDV,N) on return from SGEEV, if the user has set JOB = 0 V is not referenced. = nonzero the N eigenvectors of A are stored in the first N columns of V. See also INFO below. (Note that if the input matrix A is nearly degenerate, V may be badly conditioned, i. e. , may have nearly dependent columns. |
| 6 | `LDV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER set by the user to the leading dimension of the array V if JOB is also set nonzero. In that case, N must be. LE. LDV. If JOB is set to zero, LDV is not referenced. |
| 7 | `WORK` | `workspace-output` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(2N) temporary storage vector. Contents changed by SGEEV. |
| 8 | `JOB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER set by the user to = 0 eigenvalues only to be calculated by SGEEV. Neither V nor LDV is referenced. = nonzero eigenvalues and vectors to be calculated. In this case, A & V must be distinct arrays. Also, if LDA. GT. |
| 9 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER on return from SGEEV the value of INFO is = 0 normal return, calculation successful. = K if the eigenvalue iteration fails to converge, eigenvalues K+1 through N are correct, but no eigenvectors were computed even if they were requested (JOB nonzero). |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 normal return, calculation successful. = K if the eigenvalue iteration fails to converge, eigenvalues K+1 through N are correct, but no eigenvectors were computed even if they were requested (JOB nonzero). No. 1 recoverable N is greater than LDA No. 2 recoverable N is less than one. No. 3 recoverable JOB is nonzero and N is greater than LDV No. 4 warning LDA > LDV, elements of A other than the N by N input elements have been changed. No. 5 warning LDA < LDV, elements of V other than the N x N output elements have been changed. |

### Storage and workspace requirements

`WORK`: REAL(2N) temporary storage vector. Contents changed by SGEEV.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::sgeev`. Native symbol: `sgeev_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::sgeev`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
