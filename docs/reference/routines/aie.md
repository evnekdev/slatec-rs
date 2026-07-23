# AIE

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the Airy function for a negative argument and an exponentially scaled Airy function for a non-negative argument.

## Description

AIE(X) computes the exponentially scaled Airy function for non-negative X. It evaluates AI(X) for X .LE. 0.0 and EXP(ZETA)*AI(X) for X .GE. 0.0 where ZETA = (2.0/3.0)*(X**1.5). Series for AIF on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.09E-19 log weighted error 18.96 significant figures required 17.76 decimal places required 19.44 Series for AIG on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.51E-17 log weighted error 16.82 significant figures required 15.19 decimal places required 17.27 Series for AIP on the interval 0. to 1.00000D+00 with weighted error 5.10E-17 log weighted error 16.29 significant figures required 14.41 decimal places required 17.06

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10D`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::airy::airy_ai_scaled_f32`

## Providers

- Canonical provider: `fnlib/aie.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/aie.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/aie.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [AIE](https://www.netlib.org/slatec/fnlib/aie.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | Input value at which the source-defined function is evaluated: Calculate the Airy function for a negative argument and an exponentially scaled Airy function for a non-negative argument |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `unavailable`.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::airy::aie`. Native symbol: `aie_`. Declaration feature: `special-airy`. Provider feature: `special-airy`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::special::airy::aie`
- Public declaration feature: `special-airy`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::special::airy::airy_ai_scaled_f32`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
