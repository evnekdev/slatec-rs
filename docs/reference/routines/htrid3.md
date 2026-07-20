# HTRID3

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Reduce a complex Hermitian (packed) matrix to a real symmetric tridiagonal matrix by unitary similarity transformations.

## Description

This subroutine is a translation of a complex analogue of the ALGOL procedure TRED3, NUM. MATH. 11, 181-195(1968) by Martin, Reinsch, and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 212-226(1971). This subroutine reduces a COMPLEX HERMITIAN matrix, stored as a single square array, to a real symmetric tridiagonal matrix using unitary similarity transformations. On INPUT NM must be set to the row dimension of the two-dimensional array parameter, A, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix. N is an INTEGER variable. N must be less than or equal to NM. A contains the lower triangle of the complex Hermitian input matrix. The real parts of the matrix elements are stored in the full lower triangle of A, and the imaginary parts are stored in the transposed positions of the strict upper triangle of A. No storage is required for the zero imaginary parts of the diagonal elements. A is a twodimensional REAL array, dimensioned A(NM,N). On OUTPUT A contains some information about the unitary transformations used in the reduction. D contains the diagonal elements of the real symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). E contains the subdiagonal elements of the real tridiagonal matrix in its last N-1 positions. E(1) is set to zero. E is a one-dimensional REAL array, dimensioned E(N). E2 contains the squares of the corresponding elements of E. E2(1) is set to zero. E2 may coincide with E if the squares are not needed. E2 is a one-dimensional REAL array, dimensioned E2(N). TAU contains further information about the transformations. TAU is a one-dimensional REAL array, dimensioned TAU(2,N). Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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
- GAMS classifications: `D4C1B1`
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

- Canonical provider: `lin/htrid3.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/htrid3.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/htrid3.f) — `verified_cached`
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
