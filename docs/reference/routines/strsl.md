# STRSL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a system of the form T*X=B or TRANS(T)*X=B, where T is a triangular matrix.

## Description

STRSL solves systems of the form T * X = B or TRANS(T) * X = B where T is a triangular matrix of order N. Here TRANS(T) denotes the transpose of the matrix T. On Entry T REAL(LDT,N) T contains the matrix of the system. The zero elements of the matrix are not referenced, and the corresponding elements of the array can be used to store other information. LDT INTEGER LDT is the leading dimension of the array T. N INTEGER N is the order of the system. B REAL(N). B contains the right hand side of the system. JOB INTEGER JOB specifies what kind of system is to be solved. If JOB is 00 solve T*X=B, T lower triangular, 01 solve T*X=B, T upper triangular, 10 solve TRANS(T)*X=B, T lower triangular, 11 solve TRANS(T)*X=B, T upper triangular. On Return B B contains the solution, if INFO .EQ. 0. Otherwise B is unaltered. INFO INTEGER INFO contains zero if the system is nonsingular. Otherwise INFO contains the index of the first zero diagonal element of T.

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

- Canonical provider: `lin/strsl.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/strsl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/strsl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::strsl`
- Current legacy Rust paths: `none`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
