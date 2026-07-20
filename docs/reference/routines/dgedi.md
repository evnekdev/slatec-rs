# DGEDI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the determinant and inverse of a matrix using the factors computed by DGECO or DGEFA.

## Description

DGEDI computes the determinant and inverse of a matrix using the factors computed by DGECO or DGEFA. On Entry A DOUBLE PRECISION(LDA, N) the output from DGECO or DGEFA. LDA INTEGER the leading dimension of the array A . N INTEGER the order of the matrix A . IPVT INTEGER(N) the pivot vector from DGECO or DGEFA. WORK DOUBLE PRECISION(N) work vector. Contents destroyed. JOB INTEGER = 11 both determinant and inverse. = 01 inverse only. = 10 determinant only. On Return A inverse of original matrix if requested. Otherwise unchanged. DET DOUBLE PRECISION(2) determinant of original matrix if requested. Otherwise not referenced. Determinant = DET(1) * 10.0**DET(2) with 1.0 .LE. ABS(DET(1)) .LT. 10.0 or DET(1) .EQ. 0.0 . Error Condition A division by zero will occur if the input factor contains a zero on the diagonal and the inverse is requested. It will not occur if the subroutines are called correctly and if DGECO has set RCOND .GT. 0.0 or DGEFA has set INFO .EQ. 0 .

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
- GAMS classifications: `D3A1`
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

- Canonical provider: `lin/dgedi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dgedi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dgedi.f) — `verified_cached`
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
