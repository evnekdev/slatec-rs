# SDASTP

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Perform one step of the SDASSL integration.

## Description

SDASTP SOLVES A SYSTEM OF DIFFERENTIAL/ ALGEBRAIC EQUATIONS OF THE FORM G(X,Y,YPRIME) = 0, FOR ONE STEP (NORMALLY FROM X TO X+H). THE METHODS USED ARE MODIFIED DIVIDED DIFFERENCE,FIXED LEADING COEFFICIENT FORMS OF BACKWARD DIFFERENTIATION FORMULAS. THE CODE ADJUSTS THE STEPSIZE AND ORDER TO CONTROL THE LOCAL ERROR PER STEP. THE PARAMETERS REPRESENT X -- INDEPENDENT VARIABLE Y -- SOLUTION VECTOR AT X YPRIME -- DERIVATIVE OF SOLUTION VECTOR AFTER SUCCESSFUL STEP NEQ -- NUMBER OF EQUATIONS TO BE INTEGRATED RES -- EXTERNAL USER-SUPPLIED SUBROUTINE TO EVALUATE THE RESIDUAL. THE CALL IS CALL RES(X,Y,YPRIME,DELTA,IRES,RPAR,IPAR) X,Y,YPRIME ARE INPUT. DELTA IS OUTPUT. ON INPUT, IRES=0. RES SHOULD ALTER IRES ONLY IF IT ENCOUNTERS AN ILLEGAL VALUE OF Y OR A STOP CONDITION. SET IRES=-1 IF AN INPUT VALUE OF Y IS ILLEGAL, AND SDASTP WILL TRY TO SOLVE THE PROBLEM WITHOUT GETTING IRES = -1. IF IRES=-2, SDASTP RETURNS CONTROL TO THE CALLING PROGRAM WITH IDID = -11. JAC -- EXTERNAL USER-SUPPLIED ROUTINE TO EVALUATE THE ITERATION MATRIX (THIS IS OPTIONAL) THE CALL IS OF THE FORM CALL JAC(X,Y,YPRIME,PD,CJ,RPAR,IPAR) PD IS THE MATRIX OF PARTIAL DERIVATIVES, PD=DG/DY+CJ*DG/DYPRIME H -- APPROPRIATE STEP SIZE FOR NEXT STEP. NORMALLY DETERMINED BY THE CODE WT -- VECTOR OF WEIGHTS FOR ERROR CRITERION. JSTART -- INTEGER VARIABLE SET 0 FOR FIRST STEP, 1 OTHERWISE. IDID -- COMPLETION CODE WITH THE FOLLOWING MEANINGS: IDID= 1 -- THE STEP WAS COMPLETED SUCCESSFULLY IDID=-6 -- THE ERROR TEST FAILED REPEATEDLY IDID=-7 -- THE CORRECTOR COULD NOT CONVERGE IDID=-8 -- THE ITERATION MATRIX IS SINGULAR IDID=-9 -- THE CORRECTOR COULD NOT CONVERGE. THERE WERE REPEATED ERROR TEST FAILURES ON THIS STEP. IDID=-10-- THE CORRECTOR COULD NOT CONVERGE BECAUSE IRES WAS EQUAL TO MINUS ONE IDID=-11-- IRES EQUAL TO -2 WAS ENCOUNTERED, AND CONTROL IS BEING RETURNED TO THE CALLING PROGRAM RPAR,IPAR -- REAL AND INTEGER PARAMETER ARRAYS THAT ARE USED FOR COMMUNICATION BETWEEN THE CALLING PROGRAM AND EXTERNAL USER ROUTINES THEY ARE NOT ALTERED BY SDASTP PHI -- ARRAY OF DIVIDED DIFFERENCES USED BY SDASTP. THE LENGTH IS NEQ*(K+1),WHERE K IS THE MAXIMUM ORDER DELTA,E -- WORK VECTORS FOR SDASTP OF LENGTH NEQ WM,IWM -- REAL AND INTEGER ARRAYS STORING MATRIX INFORMATION SUCH AS THE MATRIX OF PARTIAL DERIVATIVES,PERMUTATION VECTOR, AND VARIOUS OTHER INFORMATION. THE OTHER PARAMETERS ARE INFORMATION WHICH IS NEEDED INTERNALLY BY SDASTP TO CONTINUE FROM STEP TO STEP.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
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

- Canonical provider: `main-src/src/sdastp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sdastp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sdastp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sdastp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [ODE solvers](../families/ode-solvers.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `X` | output | `REAL` (`explicit`) | `*mut f32` | scalar | SDASTP SOLVES A SYSTEM OF DIFFERENTIAL/ ALGEBRAIC EQUATIONS OF THE FORM G(X,Y,YPRIME) = 0, FOR ONE STEP (NORMALLY FROM X TO X+H). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | SDASTP SOLVES A SYSTEM OF DIFFERENTIAL/ ALGEBRAIC EQUATIONS OF THE FORM G(X,Y,YPRIME) = 0, FOR ONE STEP (NORMALLY FROM X TO X+H). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YPRIME` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | SDASTP SOLVES A SYSTEM OF DIFFERENTIAL/ ALGEBRAIC EQUATIONS OF THE FORM G(X,Y,YPRIME) = 0, FOR ONE STEP (NORMALLY FROM X TO X+H). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEQ` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | THE PARAMETERS REPRESENT X -- INDEPENDENT VARIABLE Y -- SOLUTION VECTOR AT X YPRIME -- DERIVATIVE OF SOLUTION VECTOR AFTER SUCCESSFUL STEP NEQ -- NUMBER OF EQUATIONS TO BE INTEGRATED RES -- EXTERNAL USER-SUPPLIED SUBROUTINE TO EVALUATE THE RESIDUAL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RES` | callback | `REAL` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | THE PARAMETERS REPRESENT X -- INDEPENDENT VARIABLE Y -- SOLUTION VECTOR AT X YPRIME -- DERIVATIVE OF SOLUTION VECTOR AFTER SUCCESSFUL STEP NEQ -- NUMBER OF EQUATIONS TO BE INTEGRATED RES -- EXTERNAL USER-SUPPLIED SUBROUTINE TO EVALUATE THE RESIDUAL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JAC` | callback | `INTEGER` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | JAC -- EXTERNAL USER-SUPPLIED ROUTINE TO EVALUATE THE ITERATION MATRIX (THIS IS OPTIONAL) THE CALL IS OF THE FORM CALL JAC(X,Y,YPRIME,PD,CJ,RPAR,IPAR) PD IS THE MATRIX OF PARTIAL DERIVATIVES, PD=DG/DY+CJ*DG/DYPRIME H -- APPROPRIATE STEP SIZE FOR NEXT STEP. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `H` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | SDASTP SOLVES A SYSTEM OF DIFFERENTIAL/ ALGEBRAIC EQUATIONS OF THE FORM G(X,Y,YPRIME) = 0, FOR ONE STEP (NORMALLY FROM X TO X+H). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WT` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | NORMALLY DETERMINED BY THE CODE WT -- VECTOR OF WEIGHTS FOR ERROR CRITERION. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JSTART` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | JSTART -- INTEGER VARIABLE SET 0 FOR FIRST STEP, 1 OTHERWISE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IDID` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | IF IRES=-2, SDASTP RETURNS CONTROL TO THE CALLING PROGRAM WITH IDID = -11. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RPAR` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | THE CALL IS CALL RES(X,Y,YPRIME,DELTA,IRES,RPAR,IPAR) X,Y,YPRIME ARE INPUT. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPAR` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | THE CALL IS CALL RES(X,Y,YPRIME,DELTA,IRES,RPAR,IPAR) X,Y,YPRIME ARE INPUT. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PHI` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (NEQ, *) | IDID=-10-- THE CORRECTOR COULD NOT CONVERGE BECAUSE IRES WAS EQUAL TO MINUS ONE IDID=-11-- IRES EQUAL TO -2 WAS ENCOUNTERED, AND CONTROL IS BEING RETURNED TO THE CALLING PROGRAM RPAR,IPAR -- REAL AND INTEGER PARAMETER ARRAYS THAT ARE USED FOR COMMUNICATION BETWEEN THE CALLING PROGRAM AND EXTERNAL USER ROUTINES THEY ARE NOT ALTERED BY SDASTP PHI -- ARRAY OF DIVIDED DIFFERENCES USED BY SDASTP. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `DELTA` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | THE CALL IS CALL RES(X,Y,YPRIME,DELTA,IRES,RPAR,IPAR) X,Y,YPRIME ARE INPUT. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `E` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | THE LENGTH IS NEQ*(K+1),WHERE K IS THE MAXIMUM ORDER DELTA,E -- WORK VECTORS FOR SDASTP OF LENGTH NEQ WM,IWM -- REAL AND INTEGER ARRAYS STORING MATRIX INFORMATION SUCH AS THE MATRIX OF PARTIAL DERIVATIVES,PERMUTATION VECTOR, AND VARIOUS OTHER INFORMATION. | THE LENGTH IS NEQ*(K+1),WHERE K IS THE MAXIMUM ORDER DELTA,E -- WORK VECTORS FOR SDASTP OF LENGTH NEQ WM,IWM -- REAL AND INTEGER ARRAYS STORING MATRIX INFORMATION SUCH AS THE MATRIX OF PARTIAL DERIVATIVES,PERMUTATION VECTOR, AND VARIOUS OTHER INFORMATION. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WM` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | THE LENGTH IS NEQ*(K+1),WHERE K IS THE MAXIMUM ORDER DELTA,E -- WORK VECTORS FOR SDASTP OF LENGTH NEQ WM,IWM -- REAL AND INTEGER ARRAYS STORING MATRIX INFORMATION SUCH AS THE MATRIX OF PARTIAL DERIVATIVES,PERMUTATION VECTOR, AND VARIOUS OTHER INFORMATION. | THE LENGTH IS NEQ*(K+1),WHERE K IS THE MAXIMUM ORDER DELTA,E -- WORK VECTORS FOR SDASTP OF LENGTH NEQ WM,IWM -- REAL AND INTEGER ARRAYS STORING MATRIX INFORMATION SUCH AS THE MATRIX OF PARTIAL DERIVATIVES,PERMUTATION VECTOR, AND VARIOUS OTHER INFORMATION. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IWM` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | THE LENGTH IS NEQ*(K+1),WHERE K IS THE MAXIMUM ORDER DELTA,E -- WORK VECTORS FOR SDASTP OF LENGTH NEQ WM,IWM -- REAL AND INTEGER ARRAYS STORING MATRIX INFORMATION SUCH AS THE MATRIX OF PARTIAL DERIVATIVES,PERMUTATION VECTOR, AND VARIOUS OTHER INFORMATION. | THE LENGTH IS NEQ*(K+1),WHERE K IS THE MAXIMUM ORDER DELTA,E -- WORK VECTORS FOR SDASTP OF LENGTH NEQ WM,IWM -- REAL AND INTEGER ARRAYS STORING MATRIX INFORMATION SUCH AS THE MATRIX OF PARTIAL DERIVATIVES,PERMUTATION VECTOR, AND VARIOUS OTHER INFORMATION. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ALPHA` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BETA` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `GAMMA` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PSI` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SIGMA` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `CJ` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | JAC -- EXTERNAL USER-SUPPLIED ROUTINE TO EVALUATE THE ITERATION MATRIX (THIS IS OPTIONAL) THE CALL IS OF THE FORM CALL JAC(X,Y,YPRIME,PD,CJ,RPAR,IPAR) PD IS THE MATRIX OF PARTIAL DERIVATIVES, PD=DG/DY+CJ*DG/DYPRIME H -- APPROPRIATE STEP SIZE FOR NEXT STEP. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `CJOLD` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `HOLD` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `S` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `HMIN` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `UROUND` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPHASE` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JCALC` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `K` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | THE LENGTH IS NEQ*(K+1),WHERE K IS THE MAXIMUM ORDER DELTA,E -- WORK VECTORS FOR SDASTP OF LENGTH NEQ WM,IWM -- REAL AND INTEGER ARRAYS STORING MATRIX INFORMATION SUCH AS THE MATRIX OF PARTIAL DERIVATIVES,PERMUTATION VECTOR, AND VARIOUS OTHER INFORMATION. | THE LENGTH IS NEQ*(K+1),WHERE K IS THE MAXIMUM ORDER DELTA,E -- WORK VECTORS FOR SDASTP OF LENGTH NEQ WM,IWM -- REAL AND INTEGER ARRAYS STORING MATRIX INFORMATION SUCH AS THE MATRIX OF PARTIAL DERIVATIVES,PERMUTATION VECTOR, AND VARIOUS OTHER INFORMATION. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KOLD` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NS` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NONNEG` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NTEMP` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `unsupported-abi`
- ABI validation: `pending`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-callbacks`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
