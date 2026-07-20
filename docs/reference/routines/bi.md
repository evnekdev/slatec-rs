# BI

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate the Bairy function (the Airy function of the second kind).

## Description

BI(X) calculates the Airy function of the second kind for real argument X. Series for BIF on the interval -1.00000D+00 to 1.00000D+00 with weighted error 1.88E-19 log weighted error 18.72 significant figures required 17.74 decimal places required 19.20 Series for BIG on the interval -1.00000D+00 to 1.00000D+00 with weighted error 2.61E-17 log weighted error 16.58 significant figures required 15.17 decimal places required 17.03 Series for BIF2 on the interval 1.00000D+00 to 8.00000D+00 with weighted error 1.11E-17 log weighted error 16.95 approx significant figures required 16.5 decimal places required 17.45 Series for BIG2 on the interval 1.00000D+00 to 8.00000D+00 with weighted error 1.19E-18 log weighted error 17.92 approx significant figures required 17.2 decimal places required 18.42

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
- Safe Rust paths: `slatec::special::airy::airy_bi_f32`

## Providers

- Canonical provider: `fnlib/bi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/bi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/bi.f) — `verified_cached`
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
| `X` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | BI(X) calculates the Airy function of the second kind for real argument X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Return value

The Fortran function returns `*mut f32` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `unavailable`.

### ABI and safety

Canonical path: `slatec_sys::special::airy::bi`. Native symbol: `bi_`. Feature: `special-airy`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::special::airy::bi`
- Compatibility aliases: `slatec_sys::families::special_airy::bi`
- Public declaration feature: `special-airy`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::special::airy::airy_bi_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
