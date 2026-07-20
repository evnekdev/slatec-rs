# BANDV

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Form the eigenvectors of a real symmetric band matrix associated with a set of ordered approximate eigenvalues by inverse iteration.

## Description

This subroutine finds those eigenvectors of a REAL SYMMETRIC BAND matrix corresponding to specified eigenvalues, using inverse iteration. The subroutine may also be used to solve systems of linear equations with a symmetric or non-symmetric band coefficient matrix. On INPUT NM must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix A. N is an INTEGER variable. N must be less than or equal to NM. MBW is the number of columns of the array A used to store the band matrix. If the matrix is symmetric, MBW is its (half) band width, denoted MB and defined as the number of adjacent diagonals, including the principal diagonal, required to specify the non-zero portion of the lower triangle of the matrix. If the subroutine is being used to solve systems of linear equations and the coefficient matrix is not symmetric, it must however have the same number of adjacent diagonals above the main diagonal as below, and in this case, MBW=2*MB-1. MBW is an INTEGER variable. MB must not be greater than N. A contains the lower triangle of the symmetric band input matrix stored as an N by MB array. Its lowest subdiagonal is stored in the last N+1-MB positions of the first column, its next subdiagonal in the last N+2-MB positions of the second column, further subdiagonals similarly, and finally its principal diagonal in the N positions of column MB. If the subroutine is being used to solve systems of linear equations and the coefficient matrix is not symmetric, A is N by 2*MB-1 instead with lower triangle as above and with its first superdiagonal stored in the first N-1 positions of column MB+1, its second superdiagonal in the first N-2 positions of column MB+2, further superdiagonals similarly, and finally its highest superdiagonal in the first N+1-MB positions of the last column. Contents of storage locations not part of the matrix are arbitrary. A is a two-dimensional REAL array, dimensioned A(NM,MBW). E21 specifies the ordering of the eigenvalues and contains 0.0E0 if the eigenvalues are in ascending order, or 2.0E0 if the eigenvalues are in descending order. If the subroutine is being used to solve systems of linear equations, E21 should be set to 1.0E0 if the coefficient matrix is symmetric and to -1.0E0 if not. E21 is a REAL variable. M is the number of specified eigenvalues or the number of systems of linear equations. M is an INTEGER variable. W contains the M eigenvalues in ascending or descending order. If the subroutine is being used to solve systems of linear equations (A-W(J)*I)*X(J)=B(J), where I is the identity matrix, W(J) should be set accordingly, for J=1,2,...,M. W is a one-dimensional REAL array, dimensioned W(M). Z contains the constant matrix columns (B(J),J=1,2,...,M), if the subroutine is used to solve systems of linear equations. Z is a two-dimensional REAL array, dimensioned Z(NM,M). NV must be set to the dimension of the array parameter RV as declared in the calling program dimension statement. NV is an INTEGER variable. On OUTPUT A and W are unaltered. Z contains the associated set of orthogonal eigenvectors. Any vector which fails to converge is set to zero. If the subroutine is used to solve systems of linear equations, Z contains the solution matrix columns (X(J),J=1,2,...,M). IERR is an INTEGER flag set to Zero for normal return, -J if the eigenvector corresponding to the J-th eigenvalue fails to converge, or if the J-th system of linear equations is nearly singular. RV and RV6 are temporary storage arrays. If the subroutine is being used to solve systems of linear equations, the determinant (up to sign) of A-W(M)*I is available, upon return, as the product of the first N elements of RV. RV and RV6 are one-dimensional REAL arrays. Note that RV is dimensioned RV(NV), where NV must be at least N*(2*MB-1). RV6 is dimensioned RV6(N). Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY

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
- GAMS classifications: `D4C3`
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

- Canonical provider: `lin/bandv.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/bandv.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::eigen::numerical::bandv`
- Current legacy Rust paths: `none`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
