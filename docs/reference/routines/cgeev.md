# CGEEV

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and, optionally, the eigenvectors of a complex general matrix.

## Description

CGEEV computes the eigenvalues and, optionally, the eigenvectors of a general complex matrix. Call Sequence Parameters- (The values of parameters marked with * (star) will be changed by CGEEV.)

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
- GAMS classifications: `D4A4`
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

- Canonical provider: `main-src/src/cgeev.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cgeev.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cgeev.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cgeev.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [CGEEV](https://www.netlib.org/slatec/src/cgeev.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | COMPLEX(LDA,N) complex nonsymmetric input matrix. are stored in the first N columns of V.  See also INFO below. (If the input matrix A is nearly degenerate, V will be badly conditioned, i.e. have nearly dependent columns.) must be distinct arrays. Also,  if LDA > LDV,  CGEEV changes all the elements of A thru column N.  If LDA < LDV, CGEEV changes all the elements of V through |
| 2 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER set by the user to the leading dimension of the complex array A. LDV only A(I,J) and V(I, J) for I,J = 1,...,N are changed by CGEEV. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER set by the user to the order of the matrices A and V, and the number of elements in E. are stored in the first N columns of V.  See also INFO below. (If the input matrix A is nearly degenerate, V will be badly conditioned, i.e. have nearly dependent columns.) LDV only A(I,J) and V(I, J) for I,J = 1,...,N are changed by CGEEV. by N input elements have been changed No. 5  warning      LDA < LDV,  elements of V other than the by N output elements have been changed |
| 4 | `E` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | COMPLEX(N) on return from CGEEV E contains the eigenvalues of A. |
| 5 | `V` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | COMPLEX(LDV,N) on return from CGEEV if the user has set JOB = 0        V is not referenced. is also set nonzero.  In that case N must be .LE. LDV. If JOB is set to zero LDV is not referenced. are referenced. = nonzero  eigenvalues and vectors to be calculated. must be distinct arrays. Also,  if LDA > LDV,  CGEEV changes all the elements of A thru column N.  If LDA < LDV, CGEEV changes all the elements of V through |
| 6 | `LDV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER set by the user to are referenced. = nonzero  eigenvalues and vectors to be calculated. |
| 7 | `WORK` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | REAL(3N) temporary storage vector.  Contents changed by CGEEV. |
| 8 | `JOB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is also set nonzero.  In that case N must be .LE. LDV. If JOB is set to zero LDV is not referenced. INTEGER set by the user to = 0        eigenvalues only to be calculated by CGEEV. |
| 9 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER on return from CGEEV the value of INFO is = 0  normal return, calculation successful. = K  if the eigenvalue iteration fails to converge, eigenvalues K+1 through N are correct, but no eigenvectors were computed even if they were requested (JOB nonzero). |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

No. 1  recoverable  N is greater than LDA No. 2  recoverable  N is less than one. No. 3  recoverable  JOB is nonzero and N is greater than LDV No. 4  warning      LDA > LDV,  elements of A other than the

### Storage and workspace requirements

`WORK`: REAL(3N) temporary storage vector.  Contents changed by CGEEV.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::cgeev`. Native symbol: `cgeev_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::cgeev`
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
