# CHIDI

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the determinant, inertia and inverse of a complex Hermitian matrix using the factors obtained from CHIFA.

## Description

CHIDI computes the determinant, inertia and inverse of a complex Hermitian matrix using the factors from CHIFA. On Entry

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2D1A`
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

- Canonical provider: `lin/chidi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/chidi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/chidi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [CHIDI](https://www.netlib.org/slatec/lin/chidi.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input-output` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (LDA, *) | COMPLEX(LDA,N) the output from CHIFA. contains the upper triangle of the inverse of the original matrix. The strict lower triangle is never referenced. |
| 2 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER the leading dimension of the array A. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER the order of the matrix A. |
| 4 | `KPVT` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | INTEGER(N) the pivot vector from CHIFA. |
| 5 | `DET` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (2) | REAL(2) determinant of original matrix. Determinant = DET(1) * 10. 0**DET(2) with 1. 0. LE. ABS(DET(1)). |
| 6 | `INERT` | `output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (3) | INTEGER(3) the inertia of the original matrix. number of positive eigenvalues. number of negative eigenvalues. number of zero eigenvalues. Error Condition A division by zero may occur if the inverse is requested and CHICO has set RCOND. EQ. |
| 7 | `WORK` | `workspace-output` | `workspace` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | COMPLEX(N) work vector. Contents destroyed. |
| 8 | `JOB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER JOB has the decimal expansion ABC where if C. NE. 0, the inverse is computed, if B. 0, the determinant is computed, if A. 0, the inertia is computed. For example, JOB = 111 gives all three. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and workspace requirements

`WORK`: COMPLEX(N) work vector. Contents destroyed.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::chidi`. Native symbol: `chidi_`. Declaration feature: `linear-algebra-complex`. Provider feature: `linear-algebra-complex`. ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32_array_rank1,mut_f32_array_rank1,mut_i32_array_rank1,mut_complex32_array_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::chidi`
- Public declaration feature: `linear-algebra-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
