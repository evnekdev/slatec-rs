# MINFIT

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the singular value decomposition of a rectangular matrix and solve the related linear least squares problem.

## Description

This subroutine is a translation of the ALGOL procedure MINFIT, NUM. MATH. 14, 403-420(1970) by Golub and Reinsch. HANDBOOK FOR AUTO. COMP., VOL II-LINEAR ALGEBRA, 134-151(1971). This subroutine determines, towards the solution of the linear system AX=B, the singular value decomposition A=USV of a real M by N rectangular matrix, forming U B rather than U. Householder bidiagonalization and a variant of the QR algorithm are used.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D9`
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

- Canonical provider: `lin/minfit.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/minfit.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/minfit.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [MINFIT](https://www.netlib.org/slatec/lin/minfit.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be set to the row dimension of the two-dimensional array parameters, A and B, as declared in the calling program dimension statement. Note that NM must be at least as large as the maximum of M and N. NM is an INTEGER variable. |
| 2 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of rows of A and B. M is an INTEGER variable. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of columns of A and the order of V. N is an INTEGER variable. |
| 4 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contains the rectangular coefficient matrix of the system. is a two-dimensional REAL array, dimensioned A(NM,N). |
| 5 | `W` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the N (non-negative) singular values of A (the diagonal elements of S). They are unordered. If an error exit is made, the singular values should be correct for indices IERR+1, IERR+2,. , N. W is a one-dimensional REAL array, dimensioned W(N). B has been overwritten by U B. |
| 6 | `IP` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of columns of B. IP can be zero. |
| 7 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, IP) | contains the constant column matrix of the system if IP is not zero. Otherwise, B is not referenced. B is a two- dimensional REAL array, dimensioned B(NM,IP). |
| 8 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER flag set to Zero for normal return, K if the K-th singular value has not been determined after 30 iterations. The singular values should be correct for indices IERR+1, IERR+2,. , N. |
| 9 | `RV1` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is a one-dimensional REAL array used for temporary storage, dimensioned RV1(N). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::minfit`. Native symbol: `minfit_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::minfit`
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
