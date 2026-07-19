# D1MACH

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Return floating point machine dependent constants.

## Description

D1MACH can be used to obtain machine-dependent parameters for the local machine environment. It is a function subprogram with one (input) argument, and can be referenced as follows: D = D1MACH(I) where I=1,...,5. The (output) value of D above is determined by the (input) value of I. The results for various values of I are discussed below. D1MACH( 1) = B**(EMIN-1), the smallest positive magnitude. D1MACH( 2) = B**EMAX*(1 - B**(-T)), the largest magnitude. D1MACH( 3) = B**(-T), the smallest relative spacing. D1MACH( 4) = B**(1-T), the largest relative spacing. D1MACH( 5) = LOG10(B) Assume double precision numbers are represented in the T-digit, base-B form sign (B**E)*( (X(1)/B) + ... + (X(T)/B**T) ) where 0 .LE. X(I) .LT. B for I=1,...,T, 0 .LT. X(1), and EMIN .LE. E .LE. EMAX. The values of B, T, EMIN and EMAX are provided in I1MACH as follows: I1MACH(10) = B, the base. I1MACH(14) = T, the number of base-B digits. I1MACH(15) = EMIN, the smallest exponent E. I1MACH(16) = EMAX, the largest exponent E. To alter this function for a particular environment, the desired set of DATA statements should be activated by removing the C from column 1. Also, the values of D1MACH(1) - D1MACH(4) should be checked for consistency with the local operating system.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Runtime and machine support`
- Mathematical domain: `runtime-support`
- Package provenance: `slatec-machine-constants`
- GAMS classifications: `R1`
- Family evidence: `reviewed_override` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/d1mach.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/d1mach.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/d1mach.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/d1mach.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Enriched from the 20-routine pilot; this catalogue is the canonical corpus view.
