# DFEHL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DDERKF

## Description

Fehlberg Fourth-Fifth Order Runge-Kutta Method ********************************************************************** DFEHL integrates a system of NEQ first order ordinary differential equations of the form DU/DX = DF(X,U) over one step when the vector Y(*) of initial values for U(*) and the vector YP(*) of initial derivatives, satisfying YP = DF(T,Y), are given at the starting point X=T. DFEHL advances the solution over the fixed step H and returns the fifth order (sixth order accurate locally) solution approximation at T+H in the array YS(*). F1,---,F5 are arrays of dimension NEQ which are needed for internal storage. The formulas have been grouped to control loss of significance. DFEHL should be called with an H not smaller than 13 units of roundoff in T so that the various independent arguments can be distinguished. This subroutine has been written with all variables and statement numbers entirely compatible with DRKFS. For greater efficiency, the call to DFEHL can be replaced by the module beginning with line 222 and extending to the last line just before the return statement. **********************************************************************

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DDERKF`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dfehl.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dfehl.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dfehl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
