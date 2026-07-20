# DSINH

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the double precision hyperbolic sine.

## Description

*Usage: DOUBLE PRECISION DSINH, X, Y Y = DSINH(X) *Arguments: X :IN This is the argument. It will not be modified by DSINH. *Function Return Values: DSINH : the double precision hyperbolic sine of X. *Description: DSINH evaluates the double precision hyperbolic sine of a double precision argument. SERIES FOR SINH ON THE INTERVAL 0. TO 1.00000E+00 WITH WEIGHTED ERROR 7.76E-33 LOG WEIGHTED ERROR 32.11 SIGNIFICANT FIGURES REQUIRED 31.20 DECIMAL PLACES REQUIRED 32.67

## Classification

- Historical role: `unknown`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Elementary and transcendental functions`
- Mathematical domain: `special-functions`
- Package provenance: `fnlib`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `spfun/spfun` (`special-function-documentation-or-source-candidate`)

## Official references

- Official references unavailable from current cached evidence.

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Public declaration feature: `raw-ffi-scalar-functions`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
