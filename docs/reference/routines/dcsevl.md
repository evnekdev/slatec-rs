# DCSEVL

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate a Chebyshev series.

## Description

Evaluate the N-term Chebyshev series CS at X. Adapted from a method presented in the paper by Broucke referenced below. Input Arguments --

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C3A2`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::polynomials::chebyshev::chebyshev_series`

## Providers

- Canonical provider: `fnlib/dcsevl.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/dcsevl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/dcsevl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DCSEVL](https://www.netlib.org/slatec/fnlib/dcsevl.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | value at which the series is to be evaluated. |
| 2 | `CS` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | array of N terms of a Chebyshev series. In evaluating CS, only half the first coefficient is summed. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of terms in array CS. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:f64(mut_f64,mut_f64_ptr_rank1,mut_i32)`.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::dcsevl`. Native symbol: `dcsevl_`. Declaration feature: `special`. Provider feature: `special-real`. ABI fingerprint: `function:f64(mut_f64,mut_f64_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::dcsevl`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::polynomials::chebyshev::chebyshev_series`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
