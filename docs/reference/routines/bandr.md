# BANDR

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Reduce a real symmetric band matrix to symmetric tridiagonal matrix and, optionally, accumulate orthogonal similarity transformations.

## Description

This subroutine is a translation of the ALGOL procedure BANDRD, NUM. MATH. 12, 231-241(1968) by Schwarz. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 273-283(1971). This subroutine reduces a REAL SYMMETRIC BAND matrix to a symmetric tridiagonal matrix using and optionally accumulating orthogonal similarity transformations. On INPUT NM must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix A. N is an INTEGER variable. N must be less than or equal to NM. MB is the (half) band width of the matrix, defined as the number of adjacent diagonals, including the principal diagonal, required to specify the non-zero portion of the lower triangle of the matrix. MB is less than or equal to N. MB is an INTEGER variable. A contains the lower triangle of the real symmetric band matrix. Its lowest subdiagonal is stored in the last N+1-MB positions of the first column, its next subdiagonal in the last N+2-MB positions of the second column, further subdiagonals similarly, and finally its principal diagonal in the N positions of the last column. Contents of storage locations not part of the matrix are arbitrary. A is a two-dimensional REAL array, dimensioned A(NM,MB). MATZ should be set to .TRUE. if the transformation matrix is to be accumulated, and to .FALSE. otherwise. MATZ is a LOGICAL variable. On OUTPUT A has been destroyed, except for its last two columns which contain a copy of the tridiagonal matrix. D contains the diagonal elements of the tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). E contains the subdiagonal elements of the tridiagonal matrix in its last N-1 positions. E(1) is set to zero. E is a one-dimensional REAL array, dimensioned E(N). E2 contains the squares of the corresponding elements of E. E2 may coincide with E if the squares are not needed. E2 is a one-dimensional REAL array, dimensioned E2(N). Z contains the orthogonal transformation matrix produced in the reduction if MATZ has been set to .TRUE. Otherwise, Z is not referenced. Z is a two-dimensional REAL array, dimensioned Z(NM,N). Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY ------------------------------------------------------------------

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4C1B1`
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

- Canonical provider: `lin/bandr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/bandr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [BANDR](https://www.netlib.org/slatec/lin/bandr.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `MB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `A` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | Array argument classified by fixed-form executable read/write analysis. |
| 5 | `D` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `E` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `E2` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `MATZ` | `input` | `scalar` | `LOGICAL` | `*mut crate::FortranLogical` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `Z` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | Array argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::bandr`. Native symbol: `bandr_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32,mut_f32_ptr_rank2)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::bandr`
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
