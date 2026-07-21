# RATQR

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the largest or smallest eigenvalues of a symmetric tridiagonal matrix using the rational QR method with Newton correction.

## Description

This subroutine is a translation of the ALGOL procedure RATQR, NUM. MATH. 11, 264-272(1968) by REINSCH and BAUER. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 257-265(1971). This subroutine finds the algebraically smallest or largest eigenvalues of a SYMMETRIC TRIDIAGONAL matrix by the rational QR method with Newton corrections. On Input N is the order of the matrix. N is an INTEGER variable.

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
- GAMS classifications: `D4A5`
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

- Canonical provider: `lin/ratqr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ratqr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/ratqr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [RATQR](https://www.netlib.org/slatec/lin/ratqr.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `EPS1` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `D` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `E` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 5 | `E2` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |
| 8 | `IND` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Status argument classified by fixed-form executable read/write analysis. |
| 9 | `BD` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 10 | `TYPE` | `input` | `scalar` | `LOGICAL` | `*mut crate::FortranLogical` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 11 | `IDEF` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 12 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

computed eigenvalues.  If the input EPS1 is non-positive, or indeed smaller than its default value, it is reset at each iteration to the respective default value, namely, the product of the relative machine precision and the magnitude of the current eigenvalue iterate.  The theoretical absolute K times EPS1.  EPS1 is a REAL variable. D contains the diagonal elements of the symmetric tridiagonal matrix.  D is a one-dimensional REAL array, dimensioned D(N). E contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions.  E(1) is arbitrary.  E is a one-dimensional REAL array, dimensioned E(N). E2 contains the squares of the corresponding elements of E in its last N-1 positions.  E2(1) is arbitrary.  E2 is a one- dimensional REAL array, dimensioned E2(N). M is the number of eigenvalues to be found.  M is an INTEGER variable. IDEF should be set to 1 if the input matrix is known to be positive definite, to -1 if the input matrix is known to be negative definite, and to 0 otherwise.  IDEF is an INTEGER variable. specification of IDEF, no eigenvalues are found.  If the Newton iterates for a particular eigenvalue are not monotone, the best estimate obtained is returned and IERR is set. W is a one-dimensional REAL array, dimensioned W(N).  W need not be distinct from D. IND contains in its first M positions the submatrix indices associated with the corresponding eigenvalues in W -- 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. IND is an one-dimensional INTEGER array, dimensioned IND(N). corresponding eigenvalues in W.  These bounds are usually within the tolerance specified by EPS1.  BD is a one- dimensional REAL array, dimensioned BD(N).  BD need not be distinct from E2. IERR is an INTEGER flag set to Zero       for normal return, 6*N+1      if  IDEF  is set to 1 and  TYPE  to .TRUE. when the matrix is NOT positive definite, or if  IDEF  is set to -1 and  TYPE  to .FALSE. when the matrix is NOT negative definite, no eigenvalues are computed, or M is greater than N, 5*N+K      if successive iterates to the K-th eigenvalue are NOT monotone increasing, where K refers to the last such occurrence. Note that subroutine TRIDIB is generally faster and more accurate than RATQR if the eigenvalues are clustered. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY ------------------------------------------------------------------

### Storage and workspace requirements

`W`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::ratqr`. Native symbol: `ratqr_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::ratqr`
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
