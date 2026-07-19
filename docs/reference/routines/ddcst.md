# DDCST

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

DDCST sets coefficients used by the core integrator DDSTP.

## Description

DDCST is called by DDNTL. The array EL determines the basic method. The array TQ is involved in adjusting the step size in relation to truncation error. EL and TQ depend upon MINT, and are calculated for orders 1 to MAXORD(.LE. 12). For each order NQ, the coefficients EL are calculated from the generating polynomial: L(T) = EL(1,NQ) + EL(2,NQ)*T + ... + EL(NQ+1,NQ)*T**NQ. For the implicit Adams methods, L(T) is given by dL/dT = (1+T)*(2+T)* ... *(NQ-1+T)/K, L(-1) = 0, where K = factorial(NQ-1). For the Gear methods, L(T) = (1+T)*(2+T)* ... *(NQ+T)/K, where K = factorial(NQ)*(1 + 1/2 + ... + 1/NQ). For each order NQ, there are three components of TQ.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `ode-dae-families`
- Family evidence: `description_inference` (`medium`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/ddcst.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ddcst.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ddcst.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
