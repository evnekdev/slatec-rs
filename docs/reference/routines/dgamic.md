# DGAMIC

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the complementary incomplete Gamma function.

## Description

Evaluate the complementary incomplete Gamma function DGAMIC = integral from X to infinity of EXP(-T) * T**(A-1.) . DGAMIC is evaluated for arbitrary real values of A and for non- negative values of X (even though DGAMIC is defined for X .LT. 0.0), except that for X = 0 and A .LE. 0.0, DGAMIC is undefined. DGAMIC, A, and X are DOUBLE PRECISION. A slight deterioration of 2 or 3 digits accuracy will occur when DGAMIC is very large or very small in absolute value, because log- arithmic variables are used. Also, if the parameter A is very close to a negative INTEGER (but not a negative integer), there is a loss of accuracy, which is reported if the result is less than half machine precision.

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
- GAMS classifications: `C7E`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::gamma::incomplete_gamma_upper`

## Providers

- Canonical provider: `fnlib/dgamic.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/dgamic.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/dgamic.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DGAMIC](https://www.netlib.org/slatec/fnlib/dgamic.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Input value at which the source-defined function is evaluated: Calculate the complementary incomplete Gamma function |
| 2 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Input value at which the source-defined function is evaluated: Calculate the complementary incomplete Gamma function |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `unavailable`.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::special::gamma::dgamic`. Native symbol: `dgamic_`. Declaration feature: `special-gamma`. Provider feature: `special-gamma`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::special::gamma::dgamic`
- Public declaration feature: `special-gamma`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::special::gamma::incomplete_gamma_upper`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
