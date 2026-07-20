# SSPDI

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the determinant, inertia, inverse of a real symmetric matrix stored in packed form using the factors from SSPFA.

## Description

SSPDI computes the determinant, inertia and inverse of a real symmetric matrix using the factors from SSPFA, where the matrix is stored in packed form. On Entry AP REAL (N*(N+1)/2) the output from SSPFA. N INTEGER the order of the matrix A. KPVT INTEGER(N) the pivot vector from SSPFA. WORK REAL(N) work vector. Contents ignored. JOB INTEGER JOB has the decimal expansion ABC where If C .NE. 0, the inverse is computed, If B .NE. 0, the determinant is computed, If A .NE. 0, the inertia is computed. For example, JOB = 111 gives all three. On Return Variables not requested by JOB are not used. AP contains the upper triangle of the inverse of the original matrix, stored in packed form. The columns of the upper triangle are stored sequentially in a one-dimensional array. DET REAL(2) determinant of original matrix. Determinant = DET(1) * 10.0**DET(2) with 1.0 .LE. ABS(DET(1)) .LT. 10.0 or DET(1) = 0.0. INERT INTEGER(3) the inertia of the original matrix. INERT(1) = number of positive eigenvalues. INERT(2) = number of negative eigenvalues. INERT(3) = number of zero eigenvalues. Error Condition A division by zero will occur if the inverse is requested and SSPCO has set RCOND .EQ. 0.0 or SSPFA has set INFO .NE. 0 .

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
- GAMS classifications: `D2B1A`
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

- Canonical provider: `lin/sspdi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sspdi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sspdi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [Dense linear algebra](../families/dense-linear-algebra.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `AP` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | On Entry AP REAL (N*(N+1)/2) the output from SSPFA. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | On Entry AP REAL (N*(N+1)/2) the output from SSPFA. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KPVT` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | KPVT INTEGER(N) the pivot vector from SSPFA. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `DET` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (2) | DET REAL(2) determinant of original matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INERT` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (3) | INERT INTEGER(3) the inertia of the original matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | WORK REAL(N) work vector. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JOB` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | JOB INTEGER JOB has the decimal expansion ABC where If C .NE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::packed::sspdi`. Native symbol: `sspdi_`. Feature: `linear-algebra`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::packed::sspdi`
- Compatibility aliases: `slatec_sys::linear_algebra::dense::sspdi`
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
