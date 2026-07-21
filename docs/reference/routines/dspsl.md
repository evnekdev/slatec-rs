# DSPSL

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a real symmetric system using the factors obtained from DSPFA.

## Description

DSISL solves the double precision symmetric system A * X = B using the factors computed by DSPFA. On Entry

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

- Canonical provider: `lin/dspsl.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dspsl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dspsl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DSPSL](https://www.netlib.org/slatec/lin/dspsl.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `AP` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | DOUBLE PRECISION(N*(N+1)/2) the output from DSPFA. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER the order of the matrix A. |
| 3 | `KPVT` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | INTEGER(N) the pivot vector from DSPFA. |
| 4 | `B` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | DOUBLE PRECISION(N) the right hand side vector. the solution vector X. Error Condition A division by zero may occur if DSPCO has set RCOND. EQ. 0. 0 or DSPFA has set INFO. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::packed::dspsl`. Native symbol: `dspsl_`. Declaration feature: `linear-algebra`. Provider feature: `linear-algebra-real`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_f64_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::packed::dspsl`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
