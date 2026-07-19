# STRDI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the determinant and inverse of a triangular matrix.

## Description

STRDI computes the determinant and inverse of a real triangular matrix. On Entry T REAL(LDT,N) T contains the triangular matrix. The zero elements of the matrix are not referenced, and the corresponding elements of the array can be used to store other information. LDT INTEGER LDT is the leading dimension of the array T. N INTEGER N is the order of the system. JOB INTEGER = 010 no det, inverse of lower triangular. = 011 no det, inverse of upper triangular. = 100 det, no inverse. = 110 det, inverse of lower triangular. = 111 det, inverse of upper triangular. On Return T inverse of original matrix if requested. Otherwise unchanged. DET REAL(2) determinant of original matrix if requested. Otherwise not referenced. Determinant = DET(1) * 10.0**DET(2) with 1.0 .LE. ABS(DET(1)) .LT. 10.0 or DET(1) .EQ. 0.0 . INFO INTEGER INFO contains zero if the system is nonsingular and the inverse is requested. Otherwise INFO contains the index of a zero diagonal element of T.

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
- GAMS classifications: `D2A3`
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

- Canonical provider: `lin/strdi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/strdi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/strdi.f) — `verified_cached`
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
