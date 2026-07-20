# CGEDI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the determinant and inverse of a matrix using the factors computed by CGECO or CGEFA.

## Description

CGEDI computes the determinant and inverse of a matrix using the factors computed by CGECO or CGEFA. On Entry A COMPLEX(LDA, N) the output from CGECO or CGEFA. LDA INTEGER the leading dimension of the array A . N INTEGER the order of the matrix A . IPVT INTEGER(N) the pivot vector from CGECO or CGEFA. WORK COMPLEX(N) work vector. Contents destroyed. JOB INTEGER = 11 both determinant and inverse. = 01 inverse only. = 10 determinant only. On Return A inverse of original matrix if requested. Otherwise unchanged. DET COMPLEX(2) determinant of original matrix if requested. Otherwise not referenced. Determinant = DET(1) * 10.0**DET(2) with 1.0 .LE. CABS1(DET(1)) .LT. 10.0 or DET(1) .EQ. 0.0 . Error Condition A division by zero will occur if the input factor contains a zero on the diagonal and the inverse is requested. It will not occur if the subroutines are called correctly and if CGECO has set RCOND .GT. 0.0 or CGEFA has set INFO .EQ. 0 .

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
- GAMS classifications: `D2C1`
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

- Canonical provider: `lin/cgedi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cgedi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cgedi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_c_automated_public`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cgedi`
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
