# FIGI2

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Transforms certain real non-symmetric tridiagonal matrix to symmetric tridiagonal matrix.

## Description

Given a NONSYMMETRIC TRIDIAGONAL matrix such that the products of corresponding pairs of off-diagonal elements are all non-negative, and zero only when both factors are zero, this subroutine reduces it to a SYMMETRIC TRIDIAGONAL matrix using and accumulating diagonal similarity transformations. On INPUT NM must be set to the row dimension of the two-dimensional array parameters, T and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix T. N is an INTEGER variable. N must be less than or equal to NM. T contains the nonsymmetric matrix. Its subdiagonal is stored in the last N-1 positions of the first column, its diagonal in the N positions of the second column, and its superdiagonal in the first N-1 positions of the third column. T(1,1) and T(N,3) are arbitrary. T is a two-dimensional REAL array, dimensioned T(NM,3). On OUTPUT T is unaltered. D contains the diagonal elements of the tridiagonal symmetric matrix. D is a one-dimensional REAL array, dimensioned D(N). E contains the subdiagonal elements of the tridiagonal symmetric matrix in its last N-1 positions. E(1) is not set. E is a one-dimensional REAL array, dimensioned E(N). Z contains the diagonal transformation matrix produced in the symmetrization. Z is a two-dimensional REAL array, dimensioned Z(NM,N). IERR is an INTEGER flag set to Zero for normal return, N+I if T(I,1)*T(I-1,3) is negative, 2*N+I if T(I,1)*T(I-1,3) is zero with one factor non-zero. In these cases, there does not exist a symmetrizing similarity transformation which is essential for the validity of the later eigenvector computation. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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
- GAMS classifications: `D4C1C`
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

- Canonical provider: `lin/figi2.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/figi2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::eigen::numerical::figi2`
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
