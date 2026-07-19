# SGECO

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Factor a matrix using Gaussian elimination and estimate the condition number of the matrix.

## Description

SGECO factors a real matrix by Gaussian elimination and estimates the condition of the matrix. If RCOND is not needed, SGEFA is slightly faster. To solve A*X = B , follow SGECO by SGESL. To compute INVERSE(A)*C , follow SGECO by SGESL. To compute DETERMINANT(A) , follow SGECO by SGEDI. To compute INVERSE(A) , follow SGECO by SGEDI. On Entry A REAL(LDA, N) the matrix to be factored. LDA INTEGER the leading dimension of the array A . N INTEGER the order of the matrix A . On Return A an upper triangular matrix and the multipliers which were used to obtain it. The factorization can be written A = L*U , where L is a product of permutation and unit lower triangular matrices and U is upper triangular. IPVT INTEGER(N) an integer vector of pivot indices. RCOND REAL an estimate of the reciprocal condition of A . For the system A*X = B , relative perturbations in A and B of size EPSILON may cause relative perturbations in X of size EPSILON/RCOND . If RCOND is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then A may be singular to working precision. In particular, RCOND is zero if exact singularity is detected or the estimate underflows. Z REAL(N) a work vector whose contents are usually unimportant. If A is close to a singular matrix, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z) .

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
- GAMS classifications: `D2A1`
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

- Canonical provider: `lin/sgeco.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sgeco.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sgeco.f) — `verified_cached`
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
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
