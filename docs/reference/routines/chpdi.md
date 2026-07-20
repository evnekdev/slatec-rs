# CHPDI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the determinant, inertia and inverse of a complex Hermitian matrix stored in packed form using the factors obtained from CHPFA.

## Description

CHPDI computes the determinant, inertia and inverse of a complex Hermitian matrix using the factors from CHPFA, where the matrix is stored in packed form. On Entry AP COMPLEX (N*(N+1)/2) the output from CHPFA. N INTEGER the order of the matrix A. KVPT INTEGER(N) the pivot vector from CHPFA. WORK COMPLEX(N) work vector. Contents ignored. JOB INTEGER JOB has the decimal expansion ABC where if C .NE. 0, the inverse is computed, if B .NE. 0, the determinant is computed, if A .NE. 0, the inertia is computed. For example, JOB = 111 gives all three. On Return Variables not requested by JOB are not used. AP contains the upper triangle of the inverse of the original matrix, stored in packed form. The columns of the upper triangle are stored sequentially in a one-dimensional array. DET REAL(2) determinant of original matrix. Determinant = DET(1) * 10.0**DET(2) with 1.0 .LE. ABS(DET(1)) .LT. 10.0 or DET(1) = 0.0. INERT INTEGER(3) the inertia of the original matrix. INERT(1) = number of positive eigenvalues. INERT(2) = number of negative eigenvalues. INERT(3) = number of zero eigenvalues. Error Condition A division by zero will occur if the inverse is requested and CHPCO has set RCOND .EQ. 0.0 or CHPFA has set INFO .NE. 0 .

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
- GAMS classifications: `D2D1A`
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

- Canonical provider: `lin/chpdi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/chpdi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/chpdi.f) — `verified_cached`
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
- Public declaration feature: `raw-ffi-complex-arguments`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
