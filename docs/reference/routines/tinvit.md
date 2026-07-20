# TINVIT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvectors of symmetric tridiagonal matrix corresponding to specified eigenvalues, using inverse iteration.

## Description

This subroutine is a translation of the inverse iteration technique in the ALGOL procedure TRISTURM by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvectors of a TRIDIAGONAL SYMMETRIC matrix corresponding to specified eigenvalues, using inverse iteration. On Input NM must be set to the row dimension of the two-dimensional array parameter, Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix. N is an INTEGER variable. N must be less than or equal to NM. D contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). E contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned E(N). E2 contains the squares of the corresponding elements of E, with zeros corresponding to negligible elements of E. E(I) is considered negligible if it is not larger than the product of the relative machine precision and the sum of the magnitudes of D(I) and D(I-1). E2(1) must contain 0.0e0 if the eigenvalues are in ascending order, or 2.0e0 if the eigenvalues are in descending order. If BISECT, TRIDIB, or IMTQLV has been used to find the eigenvalues, their output E2 array is exactly what is expected here. E2 is a one-dimensional REAL array, dimensioned E2(N). M is the number of specified eigenvalues for which eigenvectors are to be determined. M is an INTEGER variable. W contains the M eigenvalues in ascending or descending order. W is a one-dimensional REAL array, dimensioned W(M). IND contains in its first M positions the submatrix indices associated with the corresponding eigenvalues in W -1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. If BISECT or TRIDIB has been used to determine the eigenvalues, their output IND array is suitable for input to TINVIT. IND is a one-dimensional INTEGER array, dimensioned IND(M). On Output ** All input arrays are unaltered.** Z contains the associated set of orthonormal eigenvectors. Any vector which fails to converge is set to zero. Z is a two-dimensional REAL array, dimensioned Z(NM,M). IERR is an INTEGER flag set to Zero for normal return, -J if the eigenvector corresponding to the J-th eigenvalue fails to converge in 5 iterations. RV1, RV2 and RV3 are one-dimensional REAL arrays used for temporary storage. They are used to store the main diagonal and the two adjacent diagonals of the triangular matrix produced in the inverse iteration process. RV1, RV2 and RV3 are dimensioned RV1(N), RV2(N) and RV3(N). RV4 and RV6 are one-dimensional REAL arrays used for temporary storage. RV4 holds the multipliers of the Gaussian elimination process. RV6 holds the approximate eigenvectors in this process. RV4 and RV6 are dimensioned RV4(N) and RV6(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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

- Canonical provider: `lin/tinvit.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/tinvit.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
