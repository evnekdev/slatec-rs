# BQR

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute some of the eigenvalues of a real symmetric matrix using the QR method with shifts of origin.

## Description

This subroutine is a translation of the ALGOL procedure BQR, NUM. MATH. 16, 85-92(1970) by Martin, Reinsch, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL II-LINEAR ALGEBRA, 266-272(1971). This subroutine finds the eigenvalue of smallest (usually) magnitude of a REAL SYMMETRIC BAND matrix using the QR algorithm with shifts of origin. Consecutive calls can be made to find further eigenvalues.

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
- GAMS classifications: `D4A6`
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

- Canonical provider: `lin/bqr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/bqr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/bqr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [BQR](https://www.netlib.org/slatec/lin/bqr.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be set to the row dimension of the two-dimensional array parameter, A, as declared in the calling program dimension statement. NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrix A. N is an INTEGER variable. must be less than or equal to NM. |
| 3 | `MB` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the (half) band width of the matrix, defined as the number of adjacent diagonals, including the principal diagonal, required to specify the non-zero portion of the lower triangle of the matrix. MB is an INTEGER variable. must be less than or equal to N on first call. should not be altered even when it exceeds the current N. Calls PYTHAG(A,B) for SQRT(A**2 + B**2). Questions and comments should be directed to B. |
| 4 | `A` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contains the lower triangle of the symmetric band input matrix stored as an N by MB array. Its lowest subdiagonal is stored in the last N+1-MB positions of the first column, its next subdiagonal in the last N+2-MB positions of the second column, further subdiagonals similarly, and finally its principal diagonal in the N positions of the last column. Contents of storages not part of the matrix are arbitrary. On a subsequent call, its output contents from the previous call should be passed. A is a two-dimensional REAL array, dimensioned A(NM,MB). contains the transformed band matrix. |
| 5 | `T` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | specifies the shift (of eigenvalues) applied to the diagonal of A in forming the input matrix. What is actually determined is the eigenvalue of A+TI (I is the identity matrix) nearest to T. On a subsequent call, the output value of T from the previous call should be passed if the next nearest eigenvalue is sought. T is a REAL variable. contains the computed eigenvalue of A+TI (if IERR is zero), where I is the identity matrix. |
| 6 | `R` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | should be specified as zero on the first call, and as its output value from the previous call on a subsequent call. It is used to determine when the last row and column of the transformed band matrix can be regarded as negligible. is a REAL variable. contains the maximum of its input value and the norm of the last column of the input matrix A. |
| 7 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER flag set to Zero for normal return, J if the J-th eigenvalue has not been determined after a total of 30 iterations. |
| 8 | `NV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be set to the dimension of the array parameter RV as declared in the calling program dimension statement. is an INTEGER variable. |
| 9 | `RV` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is a one-dimensional REAL array of dimension NV which is at least (2*MB**2+4*MB-3), used for temporary storage. The first (3*MB-2) locations correspond to the ALGOL array B, the next (2*MB-1) locations correspond to the ALGOL array H, and the final (2*MB**2-MB) locations correspond to the MB by (2*MB-1) ALGOL array U. NOTE. For a subsequent call, N should be replaced by N-1, but. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::bqr`. Native symbol: `bqr_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::bqr`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
