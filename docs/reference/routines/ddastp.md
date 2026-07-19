# DDASTP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Perform one step of the DDASSL integration.

## Description

DDASTP SOLVES A SYSTEM OF DIFFERENTIAL/ ALGEBRAIC EQUATIONS OF THE FORM G(X,Y,YPRIME) = 0, FOR ONE STEP (NORMALLY FROM X TO X+H). THE METHODS USED ARE MODIFIED DIVIDED DIFFERENCE,FIXED LEADING COEFFICIENT FORMS OF BACKWARD DIFFERENTIATION FORMULAS. THE CODE ADJUSTS THE STEPSIZE AND ORDER TO CONTROL THE LOCAL ERROR PER STEP. THE PARAMETERS REPRESENT X -- INDEPENDENT VARIABLE Y -- SOLUTION VECTOR AT X YPRIME -- DERIVATIVE OF SOLUTION VECTOR AFTER SUCCESSFUL STEP NEQ -- NUMBER OF EQUATIONS TO BE INTEGRATED RES -- EXTERNAL USER-SUPPLIED SUBROUTINE TO EVALUATE THE RESIDUAL. THE CALL IS CALL RES(X,Y,YPRIME,DELTA,IRES,RPAR,IPAR) X,Y,YPRIME ARE INPUT. DELTA IS OUTPUT. ON INPUT, IRES=0. RES SHOULD ALTER IRES ONLY IF IT ENCOUNTERS AN ILLEGAL VALUE OF Y OR A STOP CONDITION. SET IRES=-1 IF AN INPUT VALUE OF Y IS ILLEGAL, AND DDASTP WILL TRY TO SOLVE THE PROBLEM WITHOUT GETTING IRES = -1. IF IRES=-2, DDASTP RETURNS CONTROL TO THE CALLING PROGRAM WITH IDID = -11. JAC -- EXTERNAL USER-SUPPLIED ROUTINE TO EVALUATE THE ITERATION MATRIX (THIS IS OPTIONAL) THE CALL IS OF THE FORM CALL JAC(X,Y,YPRIME,PD,CJ,RPAR,IPAR) PD IS THE MATRIX OF PARTIAL DERIVATIVES, PD=DG/DY+CJ*DG/DYPRIME H -- APPROPRIATE STEP SIZE FOR NEXT STEP. NORMALLY DETERMINED BY THE CODE WT -- VECTOR OF WEIGHTS FOR ERROR CRITERION. JSTART -- INTEGER VARIABLE SET 0 FOR FIRST STEP, 1 OTHERWISE. IDID -- COMPLETION CODE WITH THE FOLLOWING MEANINGS: IDID= 1 -- THE STEP WAS COMPLETED SUCCESSFULLY IDID=-6 -- THE ERROR TEST FAILED REPEATEDLY IDID=-7 -- THE CORRECTOR COULD NOT CONVERGE IDID=-8 -- THE ITERATION MATRIX IS SINGULAR IDID=-9 -- THE CORRECTOR COULD NOT CONVERGE. THERE WERE REPEATED ERROR TEST FAILURES ON THIS STEP. IDID=-10-- THE CORRECTOR COULD NOT CONVERGE BECAUSE IRES WAS EQUAL TO MINUS ONE IDID=-11-- IRES EQUAL TO -2 WAS ENCOUNTERED, AND CONTROL IS BEING RETURNED TO THE CALLING PROGRAM RPAR,IPAR -- REAL AND INTEGER PARAMETER ARRAYS THAT ARE USED FOR COMMUNICATION BETWEEN THE CALLING PROGRAM AND EXTERNAL USER ROUTINES THEY ARE NOT ALTERED BY DDASTP PHI -- ARRAY OF DIVIDED DIFFERENCES USED BY DDASTP. THE LENGTH IS NEQ*(K+1),WHERE K IS THE MAXIMUM ORDER DELTA,E -- WORK VECTORS FOR DDASTP OF LENGTH NEQ WM,IWM -- REAL AND INTEGER ARRAYS STORING MATRIX INFORMATION SUCH AS THE MATRIX OF PARTIAL DERIVATIVES,PERMUTATION VECTOR, AND VARIOUS OTHER INFORMATION. THE OTHER PARAMETERS ARE INFORMATION WHICH IS NEEDED INTERNALLY BY DDASTP TO CONTINUE FROM STEP TO STEP.

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

- Canonical provider: `main-src/src/ddastp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ddastp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ddastp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ddastp.f) — `verified_cached`
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
