# INVIT

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvectors of a real upper Hessenberg matrix associated with specified eigenvalues by inverse iteration.

## Description

This subroutine is a translation of the ALGOL procedure INVIT by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvectors of a REAL UPPER Hessenberg matrix corresponding to specified eigenvalues, using inverse iteration.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `integer_or_index`
- Scalar kind: `integer`
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

- Canonical provider: `lin/invit.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/invit.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/invit.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [INVIT](https://www.netlib.org/slatec/lin/invit.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrix A. N is an INTEGER variable. must be less than or equal to NM. |
| 3 | `A` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contains the upper Hessenberg matrix. A is a two-dimensional REAL array, dimensioned A(NM,N). unaltered. WR may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors. SELECT may have been altered. If the elements corresponding to a pair of conjugate complex eigenvalues were each initially set to. |
| 4 | `WR` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | the real and imaginary parts, respectively, of the eigenvalues of the Hessenberg matrix. The eigenvalues must be stored in a manner identical to that output by subroutine HQR, which recognizes possible splitting of the matrix. WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N). |
| 5 | `WI` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | the real and imaginary parts, respectively, of the eigenvalues of the Hessenberg matrix. The eigenvalues must be stored in a manner identical to that output by subroutine HQR, which recognizes possible splitting of the matrix. WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N). unaltered. WR may have been altered since close eigenvalues are perturbed slightly in searching for independent eigenvectors. SELECT may have been altered. |
| 6 | `SELECT` | `input` | `array` | `LOGICAL` | `*mut crate::FortranLogical` | rank 1; dimensions (N) | specifies the eigenvectors to be found. The eigenvector corresponding to the J-th eigenvalue is specified by setting SELECT(J) to. TRUE. SELECT is a one-dimensional LOGICAL array, dimensioned SELECT(N). |
| 7 | `MM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | should be set to an upper bound for the number of columns required to store the eigenvectors to be found. NOTE that two columns are required to store the eigenvector corresponding to a complex eigenvalue. One column is required to store the eigenvector corresponding to a real eigenvalue. MM is an INTEGER variable. |
| 8 | `M` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of columns actually used to store the eigenvectors. M is an INTEGER variable. |
| 9 | `Z` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contains the real and imaginary parts of the eigenvectors. The eigenvectors are packed into the columns of Z starting at the first column. If the next selected eigenvalue is real, the next column of Z contains its eigenvector. If the eigenvalue is complex, the next two columns of Z contain the real and imaginary parts of its eigenvector, with the real part first. The eigenvectors are normalized so that the component of largest magnitude is 1. Any vector which fails the acceptance test is set to zero. |
| 10 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER flag set to Zero for normal return, -(2*N+1) if more than MM columns of Z are necessary to store the eigenvectors corresponding to the specified eigenvalues (in this case, M is equal to the number of columns of Z containing eigenvectors already computed), -K if the iteration corresponding to the K-th value fails (if this occurs more than once, K is the index of the last occurrence); the corresponding columns of Z are set to zero vectors, -(N+K) if both error situations occur. |
| 11 | `RM1` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (N, *) | is a two-dimensional REAL array used for temporary storage. This array holds the triangularized form of the upper Hessenberg matrix used in the inverse iteration process. is dimensioned RM1(N,N). |
| 12 | `RV1` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | one-dimensional REAL arrays used for temporary storage. They hold the approximate eigenvectors during the inverse iteration process. RV1 and RV2 are dimensioned RV1(N) and RV2(N). The ALGOL procedure GUESSVEC appears in INVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. |
| 13 | `RV2` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | one-dimensional REAL arrays used for temporary storage. They hold the approximate eigenvectors during the inverse iteration process. RV1 and RV2 are dimensioned RV1(N) and RV2(N). The ALGOL procedure GUESSVEC appears in INVIT in-line. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Calls CDIV for complex division. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::invit`. Native symbol: `invit_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::invit`
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
