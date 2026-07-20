# TSTURM

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Find those eigenvalues of a symmetric tridiagonal matrix in a given interval and their associated eigenvectors by Sturm sequencing.

## Description

This subroutine finds those eigenvalues of a TRIDIAGONAL SYMMETRIC matrix which lie in a specified interval and their associated eigenvectors, using bisection and inverse iteration. On Input NM must be set to the row dimension of the two-dimensional array parameter, Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix. N is an INTEGER variable. N must be less than or equal to NM. EPS1 is an absolute error tolerance for the computed eigenvalues. It should be chosen so that the accuracy of these eigenvalues is commensurate with relative perturbations of the order of the relative machine precision in the matrix elements. If the input EPS1 is non-positive, it is reset for each submatrix to a default value, namely, minus the product of the relative machine precision and the 1-norm of the submatrix. EPS1 is a REAL variable. D contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). E contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned E(N). E2 contains the squares of the corresponding elements of E. E2(1) is arbitrary. E2 is a one-dimensional REAL array, dimensioned E2(N). LB and UB define the interval to be searched for eigenvalues. If LB is not less than UB, no eigenvalues will be found. LB and UB are REAL variables. MM should be set to an upper bound for the number of eigenvalues in the interval. MM is an INTEGER variable. WARNING - If more than MM eigenvalues are determined to lie in the interval, an error return is made with no values or vectors found. On Output EPS1 is unaltered unless it has been reset to its (last) default value. D and E are unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. E2(1) is also set to zero. M is the number of eigenvalues determined to lie in (LB,UB). M is an INTEGER variable. W contains the M eigenvalues in ascending order if the matrix does not split. If the matrix splits, the eigenvalues are in ascending order for each submatrix. If a vector error exit is made, W contains those values already found. W is a one-dimensional REAL array, dimensioned W(MM). Z contains the associated set of orthonormal eigenvectors. If an error exit is made, Z contains those vectors already found. Z is a one-dimensional REAL array, dimensioned Z(NM,MM). IERR is an INTEGER flag set to Zero for normal return, 3*N+1 if M exceeds MM no eigenvalues or eigenvectors are computed, 4*N+J if the eigenvector corresponding to the J-th eigenvalue fails to converge in 5 iterations, then the eigenvalues and eigenvectors in W and Z should be correct for indices 1, 2, ..., J-1. RV1, RV2, RV3, RV4, RV5, and RV6 are temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and RV6(N). The ALGOL procedure STURMCNT contained in TRISTURM appears in TSTURM in-line. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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

- Canonical provider: `lin/tsturm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/tsturm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/tsturm.f) — `verified_cached`
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
