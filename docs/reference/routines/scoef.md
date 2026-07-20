# SCOEF

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to BVSUP

## Description

********************************************************************** INPUT TO SCOEF ********************************************************************** YH = Matrix of homogeneous solutions. YP = Vector containing particular solution. NCOMP = Number of components per solution vector. NROWB = First dimension of B in calling program. NFC = Number of base solution vectors. NFCC = 2*NFC for the special treatment of complex valued equations. Otherwise, NFCC=NFC. NIC = Number of specified initial conditions. B = Boundary condition matrix at X = Xfinal. BETA = Vector of nonhomogeneous boundary conditions at X = Xfinal. 1 - Nonzero particular solution INHOMO = 2 - Zero particular solution 3 - Eigenvalue problem RE = Relative error tolerance AE = Absolute error tolerance BY = Storage space for the matrix B*YH CVEC = Storage space for the vector BETA-B*YP WORK = Real array of internal storage. Dimension must be .GE. NFCC*(NFCC+4) IWORK = Integer array of internal storage. Dimension must be .GE. 3+NFCC ********************************************************************** OUTPUT FROM SCOEF ********************************************************************** COEF = Array containing superposition constants. IFLAG = Indicator of success from SUDS in solving the boundary equations = 0 Boundary equations are solved = 1 Boundary equations appear to have many solutions = 2 Boundary equations appear to be inconsistent = 3 For this value of an eigenparameter, the boundary equations have only the zero solution. ********************************************************************** Subroutine SCOEF solves for the superposition constants from the linear equations defined by the boundary conditions at X = Xfinal. B*YP + B*YH*COEF = BETA **********************************************************************

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `BVSUP`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/scoef.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/scoef.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/scoef.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/scoef.f) — `verified_cached`
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
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
