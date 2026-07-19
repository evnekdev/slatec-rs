# CHFCM

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Check a single cubic for monotonicity.

## Description

*Usage: REAL D1, D2, DELTA INTEGER ISMON, CHFCM ISMON = CHFCM (D1, D2, DELTA) *Arguments: D1,D2:IN are the derivative values at the ends of an interval. DELTA:IN is the data slope over that interval. *Function Return Values: ISMON : indicates the monotonicity of the cubic segment: ISMON = -3 if function is probably decreasing; ISMON = -1 if function is strictly decreasing; ISMON = 0 if function is constant; ISMON = 1 if function is strictly increasing; ISMON = 2 if function is non-monotonic; ISMON = 3 if function is probably increasing. If ABS(ISMON)=3, the derivative values are too close to the boundary of the monotonicity region to declare monotonicity in the presence of roundoff error. *Description: CHFCM: Cubic Hermite Function -- Check Monotonicity. Called by PCHCM to determine the monotonicity properties of the cubic with boundary derivative values D1,D2 and chord slope DELTA. *Cautions: This is essentially the same as old CHFMC, except that a new output value, -3, was added February 1989. (Formerly, -3 and +3 were lumped together in the single value 3.) Codes that flag nonmonotonicity by "IF (ISMON.EQ.2)" need not be changed. Codes that check via "IF (ISMON.GE.3)" should change the test to "IF (IABS(ISMON).GE.3)". Codes that declare monotonicity via "IF (ISMON.LE.1)" should change to "IF (IABS(ISMON).LE.1)". REFER TO PCHCM

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
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

- Canonical provider: `pchip/chfcm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/chfcm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/chfcm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
