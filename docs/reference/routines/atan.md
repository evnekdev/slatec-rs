# ATAN

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the arctangent.

## Description

*Usage: REAL ATAN, X, Y Y = ATAN(X) *Arguments: X :IN This is the argument. It will not be modified by ATAN. *Function Return Values: ATAN : the arctangent. *Description: ATAN evaluates the arctangent of an argument. SERIES FOR ATAN ON THE INTERVAL 0. TO 4.00000D-02 WITH WEIGHTED ERROR 1.00E-17 LOG WEIGHTED ERROR 17.00 SIGNIFICANT FIGURES REQUIRED 16.38 DECIMAL PLACES REQUIRED 17.48 XBNDN = TAN((2*N-1)*PI/16.0) TANP8(N) = TAN(N*PI/8.) CONPI8(N) + PI8(N) = N*PI/8.0

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
