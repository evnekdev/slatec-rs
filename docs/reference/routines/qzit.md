# QZIT

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The second step of the QZ algorithm for generalized eigenproblems. Accepts an upper Hessenberg and an upper triangular matrix and reduces the former to quasi-triangular form while preserving the form of the latter. Usually preceded by QZHES and followed by QZVAL and QZVEC.

## Description

This subroutine is the second step of the QZ algorithm for solving generalized matrix eigenvalue problems, SIAM J. NUMER. ANAL. 10, 241-256(1973) by MOLER and STEWART, as modified in technical note NASA TN D-7305(1973) by WARD. This subroutine accepts a pair of REAL matrices, one of them in upper Hessenberg form and the other in upper triangular form. It reduces the Hessenberg matrix to quasi-triangular form using orthogonal transformations while maintaining the triangular form of the other matrix. It is usually preceded by QZHES and followed by QZVAL and, possibly, QZVEC. On Input NM must be set to the row dimension of the two-dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrices A and B. N is an INTEGER variable. N must be less than or equal to NM. A contains a real upper Hessenberg matrix. A is a two- dimensional REAL array, dimensioned A(NM,N). B contains a real upper triangular matrix. B is a two- dimensional REAL array, dimensioned B(NM,N). EPS1 is a tolerance used to determine negligible elements. EPS1 = 0.0 (or negative) may be input, in which case an element will be neglected only if it is less than roundoff

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
- GAMS classifications: `D4C1B3`
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

- Canonical provider: `lin/qzit.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/qzit.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [QZIT](https://www.netlib.org/slatec/lin/qzit.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `A` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `B` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | Array argument classified by fixed-form executable read/write analysis. |
| 5 | `EPS1` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `MATZ` | `input` | `scalar` | `LOGICAL` | `*mut crate::FortranLogical` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `Z` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

positive, then an element will be considered negligible if it is less than EPS1 times the norm of its matrix.  A positive value of EPS1 may result in faster execution, but less accurate results.  EPS1 is a REAL variable. MATZ should be set to .TRUE. if the right hand transformations are to be accumulated for later use in computing eigenvectors, and to .FALSE. otherwise.  MATZ is a LOGICAL variable. Z contains, if MATZ has been set to .TRUE., the transformation matrix produced in the reduction by  QZHES, if performed, or else the identity matrix.  If MATZ has been set to .FALSE., Z is not referenced.  Z is a two-dimensional REAL array, dimensioned Z(NM,N). On Output A has been reduced to quasi-triangular form.  The elements below the first subdiagonal are still zero, and no two consecutive subdiagonal elements are nonzero. B is still in upper triangular form, although its elements have been altered.  The location B(N,1) is used to store EPS1 times the norm of B for later use by  QZVAL  and  QZVEC. Z contains the product of the right hand transformations (for both steps) if MATZ has been set to .TRUE. IERR is an INTEGER flag set to Zero       for normal return, J          if neither A(J,J-1) nor A(J-1,J-2) has become zero after a total of 30*N iterations. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY ------------------------------------------------------------------

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::qzit`. Native symbol: `qzit_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32,mut_fortran_logical_i32,mut_f32_ptr_rank2,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::qzit`
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
