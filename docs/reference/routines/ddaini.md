# DDAINI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Initialization routine for DDASSL.

## Description

DDAINI TAKES ONE STEP OF SIZE H OR SMALLER WITH THE BACKWARD EULER METHOD, TO FIND YPRIME. X AND Y ARE UPDATED TO BE CONSISTENT WITH THE NEW STEP. A MODIFIED DAMPED NEWTON ITERATION IS USED TO SOLVE THE CORRECTOR ITERATION. THE INITIAL GUESS FOR YPRIME IS USED IN THE PREDICTION, AND IN FORMING THE ITERATION MATRIX, BUT IS NOT INVOLVED IN THE ERROR TEST. THIS MAY HAVE TROUBLE CONVERGING IF THE INITIAL GUESS IS NO GOOD, OR IF G(X,Y,YPRIME) DEPENDS NONLINEARLY ON YPRIME. THE PARAMETERS REPRESENT: X -- INDEPENDENT VARIABLE Y -- SOLUTION VECTOR AT X YPRIME -- DERIVATIVE OF SOLUTION VECTOR NEQ -- NUMBER OF EQUATIONS H -- STEPSIZE. IMDER MAY USE A STEPSIZE SMALLER THAN H. WT -- VECTOR OF WEIGHTS FOR ERROR CRITERION IDID -- COMPLETION CODE WITH THE FOLLOWING MEANINGS IDID= 1 -- YPRIME WAS FOUND SUCCESSFULLY IDID=-12 -- DDAINI FAILED TO FIND YPRIME RPAR,IPAR -- REAL AND INTEGER PARAMETER ARRAYS THAT ARE NOT ALTERED BY DDAINI PHI -- WORK SPACE FOR DDAINI DELTA,E -- WORK SPACE FOR DDAINI WM,IWM -- REAL AND INTEGER ARRAYS STORING MATRIX INFORMATION

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

- Canonical provider: `main-src/src/ddaini.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ddaini.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ddaini.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ddaini.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
