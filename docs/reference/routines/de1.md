# DE1

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the exponential integral E1(X).

## Description

DE1 calculates the double precision exponential integral, E1(X), for positive double precision argument X and the Cauchy principal value for negative X. If principal values are used everywhere, then, for all X, E1(X) = -Ei(-X) or Ei(X) = -E1(-X). Series for AE10 on the interval -3.12500E-02 to 0. with weighted error 4.62E-32 log weighted error 31.34 significant figures required 29.70 decimal places required 32.18 Series for AE11 on the interval -1.25000E-01 to -3.12500E-02 with weighted error 2.22E-32 log weighted error 31.65 significant figures required 30.75 decimal places required 32.54 Series for AE12 on the interval -2.50000E-01 to -1.25000E-01 with weighted error 5.19E-32 log weighted error 31.28 significant figures required 30.82 decimal places required 32.09 Series for E11 on the interval -4.00000E+00 to -1.00000E+00 with weighted error 8.49E-34 log weighted error 33.07 significant figures required 34.13 decimal places required 33.80 Series for E12 on the interval -1.00000E+00 to 1.00000E+00 with weighted error 8.08E-33 log weighted error 32.09 approx significant figures required 30.4 decimal places required 32.79 Series for AE13 on the interval 2.50000E-01 to 1.00000E+00 with weighted error 6.65E-32 log weighted error 31.18 significant figures required 30.69 decimal places required 32.03 Series for AE14 on the interval 0. to 2.50000E-01 with weighted error 5.07E-32 log weighted error 31.30 significant figures required 30.40 decimal places required 32.20

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
- GAMS classifications: `C5`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::integrals::exponential_integral_e1`

## Providers

- Canonical provider: `fnlib/de1.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/de1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/de1.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Special functions](../families/special-functions.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `X` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | DE1 calculates the double precision exponential integral, E1(X), for positive double precision argument X and the Cauchy principal value for negative X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Return value

The Fortran function returns `*mut f64` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `function:f64(mut_f64)`.

### ABI and safety

Canonical path: `slatec_sys::special::de1`. Native symbol: `de1_`. Feature: `special`. Provider status: `selected_provider_verified`. ABI fingerprint: `function:f64(mut_f64)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::special::de1`
- Compatibility aliases: `slatec_sys::special::numerical::de1`
- Public declaration feature: `special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::special::integrals::exponential_integral_e1`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
