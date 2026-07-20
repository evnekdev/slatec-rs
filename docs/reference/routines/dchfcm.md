# DCHFCM

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Check a single cubic for monotonicity.

## Description

*Usage: DOUBLE PRECISION D1, D2, DELTA INTEGER ISMON, DCHFCM ISMON = DCHFCM (D1, D2, DELTA) *Arguments: D1,D2:IN are the derivative values at the ends of an interval. DELTA:IN is the data slope over that interval. *Function Return Values: ISMON : indicates the monotonicity of the cubic segment: ISMON = -3 if function is probably decreasing; ISMON = -1 if function is strictly decreasing; ISMON = 0 if function is constant; ISMON = 1 if function is strictly increasing; ISMON = 2 if function is non-monotonic; ISMON = 3 if function is probably increasing. If ABS(ISMON)=3, the derivative values are too close to the boundary of the monotonicity region to declare monotonicity in the presence of roundoff error. *Description: DCHFCM: Cubic Hermite Function -- Check Monotonicity. Called by DPCHCM to determine the monotonicity properties of the cubic with boundary derivative values D1,D2 and chord slope DELTA. *Cautions: This is essentially the same as old DCHFMC, except that a new output value, -3, was added February 1989. (Formerly, -3 and +3 were lumped together in the single value 3.) Codes that flag nonmonotonicity by "IF (ISMON.EQ.2)" need not be changed. Codes that check via "IF (ISMON.GE.3)" should change the test to "IF (IABS(ISMON).GE.3)". Codes that declare monotonicity via "IF (ISMON.LE.1)" should change to "IF (IABS(ISMON).LE.1)". REFER TO DPCHCM

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `PCHIP`
- Mathematical domain: `interpolation`
- Package provenance: `pchip`
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

- Canonical provider: `pchip/dchfcm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/dchfcm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/dchfcm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
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
