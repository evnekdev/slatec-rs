# COMLR2

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and eigenvectors of a complex upper Hessenberg matrix using the modified LR method.

## Description

This subroutine is a translation of the ALGOL procedure COMLR2, NUM. MATH. 16, 181-204(1970) by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 372-395(1971). This subroutine finds the eigenvalues and eigenvectors of a COMPLEX UPPER Hessenberg matrix by the modified LR method. The eigenvectors of a COMPLEX GENERAL matrix can also be found if COMHES has been used to reduce this general matrix to Hessenberg form.

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

- Canonical provider: `lin/comlr2.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/comlr2.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/comlr2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [COMLR2](https://www.netlib.org/slatec/lin/comlr2.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be set to the row dimension of the two-dimensional array parameters, HR, HI, ZR and ZI, as declared in the calling program dimension statement. NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrix H=(HR,HI). N is an INTEGER variable. N must be less than or equal to NM. |
| 3 | `LOW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | two INTEGER variables determined by the balancing subroutine CBAL. If CBAL has not been used, set LOW=1 and IGH equal to the order of the matrix, N. |
| 4 | `IGH` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | two INTEGER variables determined by the balancing subroutine CBAL. If CBAL has not been used, set LOW=1 and IGH equal to the order of the matrix, N. |
| 5 | `INT` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | contains information on the rows and columns interchanged in the reduction by COMHES, if performed. Only elements LOW through IGH are used. If you want the eigenvectors of a complex general matrix, leave INT as it came from COMHES. If the eigenvectors of the Hessenberg matrix are desired, set INT(J)=J for these elements. INT is a one-dimensional INTEGER array, dimensioned INT(IGH). |
| 6 | `HR` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | the real and imaginary parts, respectively, of the complex upper Hessenberg matrix. Their lower triangles below the subdiagonal contain the multipliers which were used in the reduction by COMHES, if performed. If the eigenvectors of a complex general matrix are desired, leave these multipliers in the lower triangles. If the eigenvectors of the Hessenberg matrix are desired, these elements must be set to zero. HR and HI are two-dimensional REAL arrays, dimensioned HR(NM,N) and. |
| 7 | `HI` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | the real and imaginary parts, respectively, of the complex upper Hessenberg matrix. Their lower triangles below the subdiagonal contain the multipliers which were used in the reduction by COMHES, if performed. If the eigenvectors of a complex general matrix are desired, leave these multipliers in the lower triangles. If the eigenvectors of the Hessenberg matrix are desired, these elements must be set to zero. HR and HI are two-dimensional REAL arrays, dimensioned HR(NM,N) and. |
| 8 | `WR` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | the real and imaginary parts, respectively, of the eigenvalues of the upper Hessenberg matrix. If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2,. , N. WR and WI are one- dimensional REAL arrays, dimensioned WR(N) and WI(N). |
| 9 | `WI` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | the real and imaginary parts, respectively, of the eigenvalues of the upper Hessenberg matrix. If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2,. , N. WR and WI are one- dimensional REAL arrays, dimensioned WR(N) and WI(N). |
| 10 | `ZR` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | the real and imaginary parts, respectively, of the eigenvectors. The eigenvectors are unnormalized. If an error exit is made, none of the eigenvectors has been found. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,N) and ZI(NM,N). |
| 11 | `ZI` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | the real and imaginary parts, respectively, of the eigenvectors. The eigenvectors are unnormalized. If an error exit is made, none of the eigenvectors has been found. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,N) and ZI(NM,N). |
| 12 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER flag set to Zero for normal return, J if the J-th eigenvalue has not been determined after a total of 30*N iterations. The eigenvalues should be correct for indices IERR+1, IERR+2,. , N, but no eigenvectors are computed. Calls CSROOT for complex square root. Calls CDIV for complex division. Questions and comments should be directed to B. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::comlr2`. Native symbol: `comlr2_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::comlr2`
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
