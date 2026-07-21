# COMQR2

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and eigenvectors of a complex upper Hessenberg matrix.

## Description

This subroutine is a translation of a unitary analogue of the ALGOL procedure COMLR2, NUM. MATH. 16, 181-204(1970) by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 372-395(1971). The unitary analogue substitutes the QR algorithm of Francis (COMP. JOUR. 4, 332-345(1962)) for the LR algorithm. This subroutine finds the eigenvalues and eigenvectors of a COMPLEX UPPER Hessenberg matrix by the QR method. The eigenvectors of a COMPLEX GENERAL matrix can also be found if CORTH has been used to reduce this general matrix to Hessenberg form.

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
- GAMS classifications: `D4C2B`
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

- Canonical provider: `lin/comqr2.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/comqr2.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/comqr2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [COMQR2](https://www.netlib.org/slatec/lin/comqr2.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | dimensional array parameters, HR, HI, ZR, and ZI, as declared in the calling program dimension statement.  NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (HR,HI).  N is an INTEGER variable.  N must be less than or equal to NM. dimensional REAL arrays, dimensioned WR(N) and WI(N). |
| 3 | `LOW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are two INTEGER variables determined by the balancing subroutine  CBAL.  If  CBAL  has not been used, 1 and IGH equal to the order of the matrix, N. are used.  If the eigenvectors of the Hessenberg matrix are desired, set ORTR(J) and |
| 4 | `IGH` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | are two INTEGER variables determined by the balancing subroutine  CBAL.  If  CBAL  has not been used, are used.  If the eigenvectors of the Hessenberg matrix are desired, set ORTR(J) and |
| 5 | `ORTR` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | formations used in the reduction by  CORTH, if performed. ORTI, and the upper Hessenberg portions of HR and HI have been destroyed. |
| 6 | `ORTI` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | formations used in the reduction by  CORTH, if performed. to 0.0E0 for these elements.  ORTR and ORTI are one-dimensional REAL arrays, dimensioned ORTR(IGH) and |
| 7 | `HR` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix.  Their lower triangles below the subdiagonal contain information about the unitary transformations used in the reduction by  CORTH, if performed.  If the eigenvectors of the Hessenberg matrix are desired, these elements may be arbitrary.  HR and HI are two-dimensional REAL arrays, dimensioned HR(NM,N) and |
| 8 | `HI` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contain the real and imaginary parts, respectively, of the complex upper Hessenberg matrix.  Their lower triangles below the subdiagonal contain information about the unitary transformations used in the reduction by  CORTH, if performed.  If the eigenvectors of the Hessenberg matrix are desired, these elements may be arbitrary.  HR and HI are two-dimensional REAL arrays, dimensioned HR(NM,N) and |
| 9 | `WR` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contain the real and imaginary parts, respectively, of the eigenvalues of the upper Hessenberg matrix.  If an dimensional REAL arrays, dimensioned WR(N) and WI(N). |
| 10 | `WI` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contain the real and imaginary parts, respectively, of the eigenvalues of the upper Hessenberg matrix.  If an dimensional REAL arrays, dimensioned WR(N) and WI(N). |
| 11 | `ZR` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contain the real and imaginary parts, respectively, of the eigenvectors.  The eigenvectors are unnormalized. If an error exit is made, none of the eigenvectors has been dimensional REAL arrays, dimensioned ZR(NM,N) and ZI(NM,N). |
| 12 | `ZI` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contain the real and imaginary parts, respectively, of the eigenvectors.  The eigenvectors are unnormalized. If an error exit is made, none of the eigenvectors has been dimensional REAL arrays, dimensioned ZR(NM,N) and ZI(NM,N). |
| 13 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | dimensional REAL arrays, dimensioned WR(N) and WI(N). dimensional REAL arrays, dimensioned WR(N) and WI(N). is an INTEGER flag set to Zero       for normal return, J          if the J-th eigenvalue has not been determined after a total of 30*N iterations. The eigenvalues should be correct for indices IERR+2, ..., N, but no eigenvectors are computed. Calls CSROOT for complex square root. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY |

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

Canonical Rust path: `slatec_sys::linear_algebra::eigen::comqr2`. Native symbol: `comqr2_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::comqr2`
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
