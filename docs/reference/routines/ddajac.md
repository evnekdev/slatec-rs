# DDAJAC

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

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

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [ODE solvers](../families/ode-solvers.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `NEQ` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | WT = VECTOR OF WEIGHTS FOR COMPUTING NORMS E = WORK SPACE (TEMPORARY) OF LENGTH NEQ WM = REAL WORK SPACE FOR MATRICES. | WT = VECTOR OF WEIGHTS FOR COMPUTING NORMS E = WORK SPACE (TEMPORARY) OF LENGTH NEQ WM = REAL WORK SPACE FOR MATRICES. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | THIS ROUTINE COMPUTES THE ITERATION MATRIX PD=DG/DY+CJ*DG/DYPRIME (WHERE G(X,Y,YPRIME)=0). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | THIS ROUTINE COMPUTES THE ITERATION MATRIX PD=DG/DY+CJ*DG/DYPRIME (WHERE G(X,Y,YPRIME)=0). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YPRIME` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | THIS ROUTINE COMPUTES THE ITERATION MATRIX PD=DG/DY+CJ*DG/DYPRIME (WHERE G(X,Y,YPRIME)=0). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `DELTA` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Y = ARRAY CONTAINING PREDICTED VALUES YPRIME = ARRAY CONTAINING PREDICTED DERIVATIVES DELTA = RESIDUAL EVALUATED AT (X,Y,YPRIME) (USED ONLY IF IWM(MTYPE)=2 OR 5) CJ = SCALAR PARAMETER DEFINING ITERATION MATRIX H = CURRENT STEPSIZE IN INTEGRATION IER = VARIABLE WHICH IS .NE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `CJ` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | THIS ROUTINE COMPUTES THE ITERATION MATRIX PD=DG/DY+CJ*DG/DYPRIME (WHERE G(X,Y,YPRIME)=0). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `H` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Y = ARRAY CONTAINING PREDICTED VALUES YPRIME = ARRAY CONTAINING PREDICTED DERIVATIVES DELTA = RESIDUAL EVALUATED AT (X,Y,YPRIME) (USED ONLY IF IWM(MTYPE)=2 OR 5) CJ = SCALAR PARAMETER DEFINING ITERATION MATRIX H = CURRENT STEPSIZE IN INTEGRATION IER = VARIABLE WHICH IS .NE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IER` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Y = ARRAY CONTAINING PREDICTED VALUES YPRIME = ARRAY CONTAINING PREDICTED DERIVATIVES DELTA = RESIDUAL EVALUATED AT (X,Y,YPRIME) (USED ONLY IF IWM(MTYPE)=2 OR 5) CJ = SCALAR PARAMETER DEFINING ITERATION MATRIX H = CURRENT STEPSIZE IN INTEGRATION IER = VARIABLE WHICH IS .NE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WT` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | WT = VECTOR OF WEIGHTS FOR COMPUTING NORMS E = WORK SPACE (TEMPORARY) OF LENGTH NEQ WM = REAL WORK SPACE FOR MATRICES. | WT = VECTOR OF WEIGHTS FOR COMPUTING NORMS E = WORK SPACE (TEMPORARY) OF LENGTH NEQ WM = REAL WORK SPACE FOR MATRICES. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `E` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | WT = VECTOR OF WEIGHTS FOR COMPUTING NORMS E = WORK SPACE (TEMPORARY) OF LENGTH NEQ WM = REAL WORK SPACE FOR MATRICES. | WT = VECTOR OF WEIGHTS FOR COMPUTING NORMS E = WORK SPACE (TEMPORARY) OF LENGTH NEQ WM = REAL WORK SPACE FOR MATRICES. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WM` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | WT = VECTOR OF WEIGHTS FOR COMPUTING NORMS E = WORK SPACE (TEMPORARY) OF LENGTH NEQ WM = REAL WORK SPACE FOR MATRICES. | WT = VECTOR OF WEIGHTS FOR COMPUTING NORMS E = WORK SPACE (TEMPORARY) OF LENGTH NEQ WM = REAL WORK SPACE FOR MATRICES. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IWM` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | HERE PD IS COMPUTED BY THE USER-SUPPLIED ROUTINE JAC IF IWM(MTYPE) IS 1 OR 4, AND IT IS COMPUTED BY NUMERICAL FINITE DIFFERENCING IF IWM(MTYPE)IS 2 OR 5 THE PARAMETERS HAVE THE FOLLOWING MEANINGS. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RES` | callback | `REAL` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | IWM = INTEGER WORK SPACE CONTAINING MATRIX INFORMATION RES = NAME OF THE EXTERNAL USER-SUPPLIED ROUTINE TO EVALUATE THE RESIDUAL FUNCTION G(X,Y,YPRIME) IRES = FLAG WHICH IS EQUAL TO ZERO IF NO ILLEGAL VALUES IN RES, AND LESS THAN ZERO OTHERWISE. | IWM = INTEGER WORK SPACE CONTAINING MATRIX INFORMATION RES = NAME OF THE EXTERNAL USER-SUPPLIED ROUTINE TO EVALUATE THE RESIDUAL FUNCTION G(X,Y,YPRIME) IRES = FLAG WHICH IS EQUAL TO ZERO IF NO ILLEGAL VALUES IN RES, AND LESS THAN ZERO OTHERWISE. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IRES` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | IWM = INTEGER WORK SPACE CONTAINING MATRIX INFORMATION RES = NAME OF THE EXTERNAL USER-SUPPLIED ROUTINE TO EVALUATE THE RESIDUAL FUNCTION G(X,Y,YPRIME) IRES = FLAG WHICH IS EQUAL TO ZERO IF NO ILLEGAL VALUES IN RES, AND LESS THAN ZERO OTHERWISE. | IWM = INTEGER WORK SPACE CONTAINING MATRIX INFORMATION RES = NAME OF THE EXTERNAL USER-SUPPLIED ROUTINE TO EVALUATE THE RESIDUAL FUNCTION G(X,Y,YPRIME) IRES = FLAG WHICH IS EQUAL TO ZERO IF NO ILLEGAL VALUES IN RES, AND LESS THAN ZERO OTHERWISE. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `UROUND` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | UROUND = THE UNIT ROUNDOFF ERROR OF THE MACHINE BEING USED. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JAC` | callback | `INTEGER` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | HERE PD IS COMPUTED BY THE USER-SUPPLIED ROUTINE JAC IF IWM(MTYPE) IS 1 OR 4, AND IT IS COMPUTED BY NUMERICAL FINITE DIFFERENCING IF IWM(MTYPE)IS 2 OR 5 THE PARAMETERS HAVE THE FOLLOWING MEANINGS. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RPAR` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPAR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
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
- Compatibility aliases: `none`
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
