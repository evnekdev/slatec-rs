# SSIFA

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Factor a real symmetric matrix by elimination with symmetric pivoting.

## Description

SSIFA factors a real symmetric matrix by elimination with symmetric pivoting. To solve A*X = B , follow SSIFA by SSISL. To compute INVERSE(A)*C , follow SSIFA by SSISL. To compute DETERMINANT(A) , follow SSIFA by SSIDI. To compute INERTIA(A) , follow SSIFA by SSIDI. To compute INVERSE(A) , follow SSIFA by SSIDI. On Entry A REAL(LDA,N) the symmetric matrix to be factored. Only the diagonal and upper triangle are used. LDA INTEGER the leading dimension of the array A . N INTEGER the order of the matrix A . On Return A a block diagonal matrix and the multipliers which were used to obtain it. The factorization can be written A = U*D*TRANS(U) where U is a product of permutation and unit upper triangular matrices , TRANS(U) is the transpose of U , and D is block diagonal with 1 by 1 and 2 by 2 blocks. KPVT INTEGER(N) an integer vector of pivot indices. INFO INTEGER = 0 normal value. = K if the K-th pivot block is singular. This is not an error condition for this subroutine, but it does indicate that SSISL or SSIDI may divide by zero if called.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2B1A`
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

- Canonical provider: `lin/ssifa.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssifa.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/ssifa.f) — `verified_cached`
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
