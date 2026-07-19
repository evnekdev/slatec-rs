# DTANH

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the double precision hyperbolic tangent.

## Description

*Usage: DOUBLE PRECISION DTANH, X, Y Y = DTANH(X) *Arguments: X :IN This is the argument. It will not be modified by DTANH. *Function Return Values: DTANH : the double precision hyperbolic tangent of X. *Description: DTANH evaluates the double precision hyperbolic tangent of a double precision argument. SERIES FOR TANH ON THE INTERVAL 0. TO 1.00000E+00 WITH WEIGHTED ERROR 9.92E-33 LOG WEIGHTED ERROR 32.00 SIGNIFICANT FIGURES REQUIRED 31.25 DECIMAL PLACES REQUIRED 32.75

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
