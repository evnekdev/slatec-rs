# SSVDC

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Perform the singular value decomposition of a rectangular matrix.

## Description

SSVDC is a subroutine to reduce a real NxP matrix X by orthogonal transformations U and V to diagonal form. The elements S(I) are the singular values of X. The columns of U are the corresponding left singular vectors, and the columns of V the right singular vectors. On Entry X REAL(LDX,P), where LDX .GE. N. X contains the matrix whose singular value decomposition is to be computed. X is destroyed by SSVDC. LDX INTEGER LDX is the leading dimension of the array X. N INTEGER N is the number of rows of the matrix X. P INTEGER P is the number of columns of the matrix X. LDU INTEGER LDU is the leading dimension of the array U. (See below). LDV INTEGER LDV is the leading dimension of the array V. (See below). WORK REAL(N) work is a scratch array. JOB INTEGER JOB controls the computation of the singular vectors. It has the decimal expansion AB with the following meaning A .EQ. 0 Do not compute the left singular vectors. A .EQ. 1 Return the N left singular vectors in U. A .GE. 2 Return the first MIN(N,P) singular vectors in U. B .EQ. 0 Do not compute the right singular vectors. B .EQ. 1 Return the right singular vectors in V. On Return S REAL(MM), where MM=MIN(N+1,P). The first MIN(N,P) entries of S contain the singular values of X arranged in descending order of magnitude. E REAL(P). E ordinarily contains zeros. However, see the discussion of INFO for exceptions. U REAL(LDU,K), where LDU .GE. N. If JOBA .EQ. 1, then K .EQ. N. If JOBA .GE. 2 , then K .EQ. MIN(N,P). U contains the matrix of right singular vectors. U is not referenced if JOBA .EQ. 0. If N .LE. P or if JOBA .EQ. 2, then U may be identified with X in the subroutine call. V REAL(LDV,P), where LDV .GE. P. V contains the matrix of right singular vectors. V is not referenced if JOB .EQ. 0. If P .LE. N, then V may be identified with X in the subroutine call. INFO INTEGER. the singular values (and their corresponding singular vectors) S(INFO+1),S(INFO+2),...,S(M) are correct (here M=MIN(N,P)). Thus if INFO .EQ. 0, all the singular values and their vectors are correct. In any event, the matrix B = TRANS(U)*X*V is the bidiagonal matrix with the elements of S on its diagonal and the elements of E on its super-diagonal (TRANS(U) is the transpose of U). Thus the singular values of X and B are the same.

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
- GAMS classifications: `D6`
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

- Canonical provider: `lin/ssvdc.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssvdc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/ssvdc.f) — `verified_cached`
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
| `X` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDX, *) | SSVDC is a subroutine to reduce a real NxP matrix X by orthogonal transformations U and V to diagonal form. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDX` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | On Entry X REAL(LDX,P), where LDX .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `P` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | On Entry X REAL(LDX,P), where LDX .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `S` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | The elements S(I) are the singular values of X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `E` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | E REAL(P). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `U` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDU, *) | SSVDC is a subroutine to reduce a real NxP matrix X by orthogonal transformations U and V to diagonal form. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDU` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | LDU INTEGER LDU is the leading dimension of the array U. | LDU INTEGER LDU is the leading dimension of the array U. Leading dimension: LDU INTEGER LDU is the leading dimension of the array U. Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `V` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDV, *) | SSVDC is a subroutine to reduce a real NxP matrix X by orthogonal transformations U and V to diagonal form. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDV` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | LDV INTEGER LDV is the leading dimension of the array V. | LDV INTEGER LDV is the leading dimension of the array V. Leading dimension: LDV INTEGER LDV is the leading dimension of the array V. Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | WORK REAL(N) work is a scratch array. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JOB` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | JOB INTEGER JOB controls the computation of the singular vectors. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | However, see the discussion of INFO for exceptions. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::dense::ssvdc`. Native symbol: `ssvdc_`. Feature: `linear-algebra`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::ssvdc`
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
