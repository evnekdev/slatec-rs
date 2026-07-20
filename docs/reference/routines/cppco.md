# CPPCO

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Factor a complex Hermitian positive definite matrix stored in packed form and estimate the condition number of the matrix.

## Description

CPPCO factors a complex Hermitian positive definite matrix stored in packed form and estimates the condition of the matrix. If RCOND is not needed, CPPFA is slightly faster. To solve A*X = B , follow CPPCO by CPPSL. To compute INVERSE(A)*C , follow CPPCO by CPPSL. To compute DETERMINANT(A) , follow CPPCO by CPPDI. To compute INVERSE(A) , follow CPPCO by CPPDI. On Entry AP COMPLEX (N*(N+1)/2) the packed form of a Hermitian matrix A . The columns of the upper triangle are stored sequentially in a one-dimensional array of length N*(N+1)/2 . See comments below for details. N INTEGER the order of the matrix A . On Return AP an upper triangular matrix R , stored in packed form, so that A = CTRANS(R)*R . If INFO .NE. 0 , the factorization is not complete. RCOND REAL an estimate of the reciprocal condition of A . For the system A*X = B , relative perturbations in A and B of size EPSILON may cause relative perturbations in X of size EPSILON/RCOND . If RCOND is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then A may be singular to working precision. In particular, RCOND is zero if exact singularity is detected or the estimate underflows. If INFO .NE. 0 , RCOND is unchanged. Z COMPLEX(N) a work vector whose contents are usually unimportant. If A is singular to working precision, then Z is an approximate null vector in the sense that NORM(A*Z) = RCOND*NORM(A)*NORM(Z) . If INFO .NE. 0 , Z is unchanged. INFO INTEGER = 0 for normal return. = K signals an error condition. The leading minor of order K is not positive definite. Packed Storage The following program segment will pack the upper triangle of a Hermitian matrix. K = 0 DO 20 J = 1, N DO 10 I = 1, J K = K + 1 AP(K) = A(I,J) 10 CONTINUE 20 CONTINUE

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2D1B`
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

- Canonical provider: `lin/cppco.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cppco.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cppco.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_c_automated_public`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cppco`
- Current legacy Rust paths: `none`
- Public declaration feature: `batch-c-linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `representative_batch_smoke_only`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
