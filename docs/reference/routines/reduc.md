# REDUC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Reduce a generalized symmetric eigenproblem to a standard symmetric eigenproblem using Cholesky factorization.

## Description

This subroutine is a translation of the ALGOL procedure REDUC1, NUM. MATH. 11, 99-110(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 303-314(1971). This subroutine reduces the generalized SYMMETRIC eigenproblem Ax=(LAMBDA)Bx, where B is POSITIVE DEFINITE, to the standard symmetric eigenproblem using the Cholesky factorization of B. On Input NM must be set to the row dimension of the two-dimensional array parameters, A and B, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrices A and B. If the Cholesky factor L of B is already available, N should be prefixed with a minus sign. N is an INTEGER variable. A and B contain the real symmetric input matrices. Only the full upper triangles of the matrices need be supplied. If N is negative, the strict lower triangle of B contains, instead, the strict lower triangle of its Cholesky factor L. A and B are two-dimensional REAL arrays, dimensioned A(NM,N) and B(NM,N). DL contains, if N is negative, the diagonal elements of L. DL is a one-dimensional REAL array, dimensioned DL(N). On Output A contains in its full lower triangle the full lower triangle of the symmetric matrix derived from the reduction to the standard form. The strict upper triangle of A is unaltered. B contains in its strict lower triangle the strict lower triangle of its Cholesky factor L. The full upper triangle of B is unaltered. DL contains the diagonal elements of L. IERR is an INTEGER flag set to Zero for normal return, 7*N+1 if B is not positive definite. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
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

- Canonical provider: `lin/reduc.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/reduc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::eigen::numerical::reduc`
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
