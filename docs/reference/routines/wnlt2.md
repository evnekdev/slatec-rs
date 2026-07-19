# WNLT2

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to WNLIT

## Description

To test independence of incoming column. Test the column IC to determine if it is linearly independent of the columns already in the basis. In the initial tri. step, we usually want the heavy weight ALAMDA to be included in the test for independence. In this case, the value of FACTOR will have been set to 1.E0 before this procedure is invoked. In the potentially rank deficient problem, the value of FACTOR will have been set to ALSQ=ALAMDA**2 to remove the effect of the heavy weight from the test for independence. Write new column as partitioned vector (A1) number of components in solution so far = NIV (A2) M-NIV components And compute SN = inverse weighted length of A1 RN = inverse weighted length of A2 Call the column independent when RN .GT. TAU*SN

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `WNLIT`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/wnlt2.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/wnlt2.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/wnlt2.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

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
