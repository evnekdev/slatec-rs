# ENORM

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SNLS1, SNLS1E, SNSQ and SNSQE

## Description

Given an N-vector X, this function calculates the Euclidean norm of X. The Euclidean norm is computed by accumulating the sum of squares in three different sums. The sums of squares for the small and large components are scaled so that no overflows occur. Non-destructive underflows are permitted. Underflows and overflows do not occur in the computation of the unscaled sum of squares for the intermediate components. The definitions of small, intermediate and large components depend on two constants, RDWARF and RGIANT. The main restrictions on these constants are that RDWARF**2 not underflow and RGIANT**2 not overflow. The constants given here are suitable for every known computer. The function statement is REAL FUNCTION ENORM(N,X) where N is a positive integer input variable. X is an input array of length N.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Shared numerical utilities`
- Mathematical domain: `data-utilities`
- Package provenance: `multiple-parent-families`
- Secondary families: `Approximation, Nonlinear equations`
- Family evidence: `parent_inheritance` (`medium`)
- Parent-family evidence: `SNLS1, SNLS1E, SNSQ, SNSQE`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/enorm.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/enorm.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/enorm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
