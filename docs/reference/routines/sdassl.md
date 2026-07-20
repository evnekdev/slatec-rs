# SDASSL

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

This code solves a system of differential/algebraic equations of the form G(T,Y,YPRIME) = 0.

## Description

*Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. NEQ:IN This is the number of equations to be solved. T:INOUT This is the current value of the independent variable. Y(*):INOUT This array contains the solution components at T. YPRIME(*):INOUT This array contains the derivatives of the solution components at T. TOUT:IN This is a point at which a solution is desired. INFO(N):IN The basic task of the code is to solve the system from T to TOUT and return an answer at TOUT. INFO is an integer array which is used to communicate exactly how you want this task to be carried out. (See below for details.) N must be greater than or equal to 15. RTOL,ATOL:INOUT These quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. You may choose them to be both scalars or else both vectors. Caution: In Fortran 77, a scalar is not the same as an array of length 1. Some compilers may object to using scalars for RTOL,ATOL. IDID:OUT This scalar quantity is an indicator reporting what the code did. You must monitor this integer variable to decide what action to take next. RWORK:WORK A real work array of length LRW which provides the code with needed storage space. LRW:IN The length of RWORK. (See below for required length.) IWORK:WORK An integer work array of length LIW which provides the code with needed storage space. LIW:IN The length of IWORK. (See below for required length.) RPAR,IPAR:IN These are real and integer parameter arrays which you can use for communication between your calling program and the RES subroutine (and the JAC subroutine) JAC:EXT This is the name of a subroutine which you may choose to provide for defining a matrix of partial derivatives described below. Quantities which may be altered by SDASSL are: T, Y(*), YPRIME(*), INFO(1), RTOL, ATOL, IDID, RWORK(*) AND IWORK(*) *Description Subroutine SDASSL uses the backward differentiation formulas of orders one through five to solve a system of the above form for Y and YPRIME. Values for Y and YPRIME at the initial time must be given as input. These values must be consistent, (that is, if T,Y,YPRIME are the given initial values, they must satisfy G(T,Y,YPRIME) = 0.). The subroutine solves the system from T to TOUT. It is easy to continue the solution to get results at additional TOUT. This is the interval mode of operation. Intermediate results can also be obtained easily by using the intermediate-output capability. The following detailed description is divided into subsections: 1. Input required for the first call to SDASSL. 2. Output after any return from SDASSL. 3. What to do to continue the integration. 4. Error messages. -------- INPUT -- WHAT TO DO ON THE FIRST CALL TO SDASSL ------------ The first call of the code is defined to be the start of each new problem. Read through the descriptions of all the following items, provide sufficient storage space for designated arrays, set appropriate variables for the initialization of the problem, and give information about how you want the problem to be solved. RES -- Provide a subroutine of the form SUBROUTINE RES(T,Y,YPRIME,DELTA,IRES,RPAR,IPAR) to define the system of differential/algebraic equations which is to be solved. For the given values of T,Y and YPRIME, the subroutine should return the residual of the differential/algebraic system DELTA = G(T,Y,YPRIME) (DELTA(*) is a vector of length NEQ which is output for RES.) Subroutine RES must not alter T,Y or YPRIME. You must declare the name RES in an external statement in your program that calls SDASSL. You must dimension Y,YPRIME and DELTA in RES. IRES is an integer flag which is always equal to zero on input. Subroutine RES should alter IRES only if it encounters an illegal value of Y or a stop condition. Set IRES = -1 if an input value is illegal, and SDASSL will try to solve the problem without getting IRES = -1. If IRES = -2, SDASSL will return control to the calling program with IDID = -11. RPAR and IPAR are real and integer parameter arrays which you can use for communication between your calling program and subroutine RES. They are not altered by SDASSL. If you do not need RPAR or IPAR, ignore these parameters by treating them as dummy arguments. If you do choose to use them, dimension them in your calling program and in RES as arrays of appropriate length. NEQ -- Set it to the number of differential equations. (NEQ .GE. 1) T -- Set it to the initial point of the integration. T must be defined as a variable. Y(*) -- Set this vector to the initial values of the NEQ solution components at the initial point. You must dimension Y of length at least NEQ in your calling program. YPRIME(*) -- Set this vector to the initial values of the NEQ first derivatives of the solution components at the initial point. You must dimension YPRIME at least NEQ in your calling program. If you do not know initial values of some of the solution components, see the explanation of INFO(11). TOUT -- Set it to the first point at which a solution is desired. You can not take TOUT = T. integration either forward in T (TOUT .GT. T) or backward in T (TOUT .LT. T) is permitted. The code advances the solution from T to TOUT using step sizes which are automatically selected so as to achieve the desired accuracy. If you wish, the code will return with the solution and its derivative at intermediate steps (intermediate-output mode) so that you can monitor them, but you still must provide TOUT in accord with the basic aim of the code. The first step taken by the code is a critical one because it must reflect how fast the solution changes near the initial point. The code automatically selects an initial step size which is practically always suitable for the problem. By using the fact that the code will not step past TOUT in the first step, you could, if necessary, restrict the length of the initial step size. For some problems it may not be permissible to integrate past a point TSTOP because a discontinuity occurs there or the solution or its derivative is not defined beyond TSTOP. When you have declared a TSTOP point (SEE INFO(4) and RWORK(1)), you have told the code not to integrate past TSTOP. In this case any TOUT beyond TSTOP is invalid

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- GAMS classifications: `I1A2`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::dassl::DaeSession::<f32, F>::advance_to`

## Providers

- Canonical provider: `main-src/src/sdassl.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sdassl.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sdassl.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sdassl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [ODE solvers](../families/ode-solvers.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `RES` | callback | `REAL` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEQ` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `T` | input/output | `REAL` (`explicit`) | `*mut f32` | scalar | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YPRIME` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TOUT` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (15) | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RTOL` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ATOL` | input/output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IDID` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RWORK` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LRW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IWORK` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LIW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RPAR` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPAR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JAC` | callback | `INTEGER` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | *Usage: EXTERNAL RES, JAC INTEGER NEQ, INFO(N), IDID, LRW, LIW, IWORK(LIW), IPAR REAL T, Y(NEQ), YPRIME(NEQ), TOUT, RTOL, ATOL, * RWORK(LRW), RPAR CALL SDASSL (RES, NEQ, T, Y, YPRIME, TOUT, INFO, RTOL, ATOL, * IDID, RWORK, LRW, IWORK, LIW, RPAR, IPAR, JAC) *Arguments: RES:EXT This is a subroutine which you provide to define the differential/algebraic system. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.

### ABI and safety

Canonical path: `slatec_sys::dassl::sdassl`. Native symbol: `sdassl_`. Feature: `dassl`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::dassl::sdassl`
- Compatibility aliases: `none`
- Public declaration feature: `dassl`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::dassl::DaeSession::<f32, F>::advance_to`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
