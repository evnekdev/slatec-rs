# DDAJAC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the iteration matrix for DDASSL and form the LU-decomposition.

## Description

THIS ROUTINE COMPUTES THE ITERATION MATRIX PD=DG/DY+CJ*DG/DYPRIME (WHERE G(X,Y,YPRIME)=0). HERE PD IS COMPUTED BY THE USER-SUPPLIED ROUTINE JAC IF IWM(MTYPE) IS 1 OR 4, AND IT IS COMPUTED BY NUMERICAL FINITE DIFFERENCING IF IWM(MTYPE)IS 2 OR 5 THE PARAMETERS HAVE THE FOLLOWING MEANINGS. Y = ARRAY CONTAINING PREDICTED VALUES YPRIME = ARRAY CONTAINING PREDICTED DERIVATIVES DELTA = RESIDUAL EVALUATED AT (X,Y,YPRIME) (USED ONLY IF IWM(MTYPE)=2 OR 5) CJ = SCALAR PARAMETER DEFINING ITERATION MATRIX H = CURRENT STEPSIZE IN INTEGRATION IER = VARIABLE WHICH IS .NE. 0 IF ITERATION MATRIX IS SINGULAR, AND 0 OTHERWISE. WT = VECTOR OF WEIGHTS FOR COMPUTING NORMS E = WORK SPACE (TEMPORARY) OF LENGTH NEQ WM = REAL WORK SPACE FOR MATRICES. ON OUTPUT IT CONTAINS THE LU DECOMPOSITION OF THE ITERATION MATRIX. IWM = INTEGER WORK SPACE CONTAINING MATRIX INFORMATION RES = NAME OF THE EXTERNAL USER-SUPPLIED ROUTINE TO EVALUATE THE RESIDUAL FUNCTION G(X,Y,YPRIME) IRES = FLAG WHICH IS EQUAL TO ZERO IF NO ILLEGAL VALUES IN RES, AND LESS THAN ZERO OTHERWISE. (IF IRES IS LESS THAN ZERO, THE MATRIX WAS NOT COMPLETED) IN THIS CASE (IF IRES .LT. 0), THEN IER = 0. UROUND = THE UNIT ROUNDOFF ERROR OF THE MACHINE BEING USED. JAC = NAME OF THE EXTERNAL USER-SUPPLIED ROUTINE TO EVALUATE THE ITERATION MATRIX (THIS ROUTINE IS ONLY USED IF IWM(MTYPE) IS 1 OR 4)

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
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/ddajac.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ddajac.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ddajac.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ddajac.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
