# TAN

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the tangent.

## Description

*Usage: DOUBLE PRECISION TAN, X, Y Y = TAN(X) *Arguments: X :IN This is the argument. It will not be modified by TAN. *Function Return Values: TAN : the tangent of X. *Description: TAN evaluates the tangent of an argument. If the magnitude of the argument X is greater than the reciprocal of D1MACH(4), an error message is generated and the routine aborts. SERIES FOR TAN ON THE INTERVAL 0. TO 6.25000D-02 WITH WEIGHTED ERROR 3.14E-18 LOG WEIGHTED ERROR 17.50 SIGNIFICANT FIGURES REQUIRED 16.59 DECIMAL PLACES REQUIRED 18.02 PI2REC = 2/PI - 0.625

## Classification

- Historical role: `unknown`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
