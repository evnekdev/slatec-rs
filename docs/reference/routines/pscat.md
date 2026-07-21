# PSCAT

[Family: Documentation and source-processing tools](../families/documentation-and-source-processing-tools.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SLPREP

## Description

Creates the data for the FCAT file consisting of two parts: 1) linked lists enabling one to obtain the various categories and subcategories present in the library being examined 2) statements from the classification file associated with the categories.

## Classification

- Historical role: `unknown`
- Program-unit kind: `subroutine`
- Identity kind: `documentation_or_tooling_program_unit`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Documentation and source-processing tools`
- Mathematical domain: `documentation-tools`
- Package provenance: `slatec-documentation-tools`
- Family evidence: `reviewed_override` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `not_bound`
- Build/profile status: `outside_current_immutable_snapshot`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `slprep/slprep` (`documentation-support-source-candidate`)

## Official references

- Official references unavailable from current cached evidence.

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_unstructured`
- Description provenance: `source_prologue`
- Assessment: meaningful selected-source prose is available for this non-public identity
- Dedicated family page: [Documentation and source-processing tools](../families/documentation-and-source-processing-tools.md)

No independently callable argument list is present in the selected interface inventory for this identity.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `historical-program`
- ABI validation: `pending`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `not_assigned`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `no` (`missing_symbol`)
- Documentation status: `not_documented`
- Compile-test status: `not_compiled`
- Link-test status: `not_tested`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `source exists but no reviewed or ABI-validated public declaration is recorded`
<!-- raw-api-status:end -->
