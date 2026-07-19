# DPOCO

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Factor a real symmetric positive definite matrix and estimate the condition of the matrix.

## Description

DPOCO factors a double precision symmetric positive definite matrix and estimates the condition of the matrix. If RCOND is not needed, DPOFA is slightly faster. To solve A*X = B , follow DPOCO by DPOSL. To compute INVERSE(A)*C , follow DPOCO by DPOSL. To compute DETERMINANT(A) , follow DPOCO by DPODI. To compute INVERSE(A) , follow DPOCO by DPODI. On Entry A DOUBLE PRECISION(LDA, N) the symmetric matrix to be factored. Only the diagonal and upper triangle are used. LDA INTEGER the leading dimension of the array A . N INTEGER the order of the matrix A . On Return A an upper triangular matrix R so that A = TRANS(R)*R where TRANS(R) is the transpose. The strict lower triangle is unaltered. If INFO .NE. 0 , the factorization is not complete. RCOND DOUBLE PRECISION an estimate of the reciprocal condition of A . For the system A*X = B , relative perturbations in A and B of size EPSILON may cause relative perturbations in X of size EPSILON/RCOND . If RCOND is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then A may be singular to working precision. In particular, RCOND is zero if exact singularity is detected or the estimate underflows. If INFO .NE. 0 , RCOND is unchanged. Z DOUBLE PRECISION(N) a work vector whose contents are usually unimportant. If A is close to a singular matrix, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z) . If INFO .NE. 0 , Z is unchanged. INFO INTEGER = 0 for normal return. = K signals an error condition. The leading minor of order K is not positive definite.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2B1B`
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

- Canonical provider: `lin/dpoco.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dpoco.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dpoco.f) — `verified_cached`
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
