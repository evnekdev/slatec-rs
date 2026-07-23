# DEBDF

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve an initial value problem in ordinary differential equations using backward differentiation formulas. It is intended primarily for stiff problems.

## Description

This is the backward differentiation code in the package of differential equation solvers DEPAC, consisting of the codes DERKF, DEABM, and DEBDF. Design of the package was by L. F. Shampine and H. A. Watts. It is documented in SAND-79-2374 , DEPAC - Design of a User Oriented Package of ODE Solvers. DEBDF is a driver for a modification of the code LSODE written by A. C. Hindmarsh Lawrence Livermore Laboratory Livermore, California 94550 ********************************************************************** ** DEPAC PACKAGE OVERVIEW ** ********************************************************************** You have a choice of three differential equation solvers from DEPAC. The following brief descriptions are meant to aid you in choosing the most appropriate code for your problem. DERKF is a fifth order Runge-Kutta code. It is the simplest of the three choices, both algorithmically and in the use of the code. DERKF is primarily designed to solve non-stiff and mildly stiff differential equations when derivative evaluations are not expensive. It should generally not be used to get high accuracy results nor answers at a great many specific points. Because DERKF has very low overhead costs, it will usually result in the least expensive integration when solving problems requiring a modest amount of accuracy and having equations that are not costly to evaluate. DERKF attempts to discover when it is not suitable for the task posed. DEABM is a variable order (one through twelve) Adams code. Its complexity lies somewhere between that of DERKF and DEBDF. DEABM is primarily designed to solve non-stiff and mildly stiff differential equations when derivative evaluations are expensive, high accuracy results are needed or answers at many specific points are required. DEABM attempts to discover when it is not suitable for the task posed. DEBDF is a variable order (one through five) backward differentiation formula code. It is the most complicated of the three choices. DEBDF is primarily designed to solve stiff differential equations at crude to moderate tolerances. If the problem is very stiff at all, DERKF and DEABM will be quite inefficient compared to DEBDF. However, DEBDF will be inefficient compared to DERKF and DEABM on non-stiff problems because it uses much more storage, has a much larger overhead, and the low order formulas will not give high accuracies efficiently. The concept of stiffness cannot be described in a few words. If you do not know the problem to be stiff, try either DERKF or DEABM. Both of these codes will inform you of stiffness when the cost of solving such problems becomes important. ********************************************************************** ** ABSTRACT ** ********************************************************************** Subroutine DEBDF uses the backward differentiation formulas of orders one through five to integrate a system of NEQ first order ordinary differential equations of the form DU/DX = F(X,U) when the vector Y(*) of initial values for U(*) at X=T is given. The subroutine integrates from T to TOUT. It is easy to continue the integration to get results at additional TOUT. This is the interval mode of operation. It is also easy for the routine to return with The solution at each intermediate step on the way to TOUT. This is the intermediate-output mode of operation. ********************************************************************** ** DESCRIPTION OF THE ARGUMENTS TO DEBDF (AN OVERVIEW) ** ********************************************************************** The Parameters are: F -- This is the name of a subroutine which you provide to define the differential equations. NEQ -- This is the number of (first order) differential equations to be integrated. T -- This is a value of the independent variable. Y(*) -- This array contains the solution components at T. TOUT -- This is a point at which a solution is desired. INFO(*) -- The basic task of the code is to integrate the differential equations from T to TOUT and return an answer at TOUT. INFO(*) is an INTEGER array which is used to communicate exactly how you want this task to be carried out. RTOL, ATOL -- These quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. You may choose them to be both scalars or else both vectors. IDID -- This scalar quantity is an indicator reporting what the code did. You must monitor this INTEGER variable to decide what action to take next. RWORK(*), LRW -- RWORK(*) is a REAL work array of length LRW which provides the code with needed storage space. IWORK(*), LIW -- IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. RPAR, IPAR -- These are REAL and INTEGER parameter arrays which you can use for communication between your calling program and the F subroutine (and the JAC subroutine). JAC -- This is the name of a subroutine which you may choose to provide for defining the Jacobian matrix of partial derivatives DF/DU. Quantities which are used as input items are NEQ, T, Y(*), TOUT, INFO(*), RTOL, ATOL, RWORK(1), LRW, IWORK(1), IWORK(2), and LIW. Quantities which may be altered by the code are T, Y(*), INFO(1), RTOL, ATOL, IDID, RWORK(*) and IWORK(*). ********************************************************************** * INPUT -- What To Do On The First Call To DEBDF * ********************************************************************** The first call of the code is defined to be the start of each new problem. Read through the descriptions of all the following items, provide sufficient storage space for designated arrays, set appropriate variables for the initialization of the problem, and give information about how you want the problem to be solved. F -- provide a subroutine of the form F(X,U,UPRIME,RPAR,IPAR) to define the system of first order differential equations which is to be solved. For the given values of X and the vector U(*)=(U(1),U(2),...,U(NEQ)) , the subroutine must evaluate the NEQ components of the system of differential equations DU/DX=F(X,U) and store the derivatives in the array UPRIME(*), that is, UPRIME(I) = * DU(I)/DX * for equations I=1,...,NEQ. Subroutine F must not alter X or U(*). You must declare the name F in an external statement in your program that calls DEBDF. You must dimension U and UPRIME in F. RPAR and IPAR are REAL and INTEGER parameter arrays which you can use for communication between your calling program and subroutine F. They are not used or altered by DEBDF. If you do not need RPAR or IPAR, ignore these parameters by treating them as dummy arguments. If you do choose to use them, dimension them in your calling program and in F as arrays of appropriate length. NEQ -- Set it to the number of differential equations. (NEQ .GE. 1) T -- Set it to the initial point of the integration. You must use a program variable for T because the code changes its value. Y(*) -- Set this vector to the initial values of the NEQ solution components at the initial point. You must dimension Y at least NEQ in your calling program. TOUT -- Set it to the first point at which a solution is desired. You can take TOUT = T, in which case the code will evaluate the derivative of the solution at T and return. Integration either forward in T (TOUT .GT. T) or backward in T (TOUT .LT. T) is permitted. The code advances the solution from T to TOUT using step sizes which are automatically selected so as to achieve the desired accuracy. If you wish, the code will return with the solution and its derivative following each intermediate step (intermediate-output mode) so that you can monitor them, but you still must provide TOUT in accord with the basic aim of the code. The first step taken by the code is a critical one because it must reflect how fast the solution changes near the initial point. The code automatically selects an initial step size which is practically always suitable for the problem. By using the fact that the code will not step past TOUT in the first step, you could, if necessary, restrict the length of the initial step size. For some problems it may not be permissible to integrate past a point TSTOP because a discontinuity occurs there or the solution or its derivative is not defined beyond TSTOP. When you have declared a TSTOP point (see INFO(4) and RWORK(1)), you have told the code not to integrate past TSTOP. In this case any TOUT beyond TSTOP is invalid

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
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
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/debdf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/debdf.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/debdf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/debdf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `support_unit_minimal`
- Description provenance: `source_prologue`
- Assessment: the support identity records its role, side-effect boundary, and non-public disposition
- Dedicated family page: [ODE solvers](../families/ode-solvers.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `F` | callback | `REAL` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEQ` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | ********************************************************************** ** ABSTRACT ** ********************************************************************** Subroutine DEBDF uses the backward differentiation formulas of orders one through five to integrate a system of NEQ first order ordinary differential equations of the form DU/DX = F(X,U) when the vector Y(*) of initial values for U(*) at X=T is given. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `T` | input | `REAL` (`implicit_rule`) | `*mut f32` | scalar | ********************************************************************** ** ABSTRACT ** ********************************************************************** Subroutine DEBDF uses the backward differentiation formulas of orders one through five to integrate a system of NEQ first order ordinary differential equations of the form DU/DX = F(X,U) when the vector Y(*) of initial values for U(*) at X=T is given. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | input | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | ********************************************************************** ** ABSTRACT ** ********************************************************************** Subroutine DEBDF uses the backward differentiation formulas of orders one through five to integrate a system of NEQ first order ordinary differential equations of the form DU/DX = F(X,U) when the vector Y(*) of initial values for U(*) at X=T is given. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TOUT` | output | `REAL` (`implicit_rule`) | `*mut f32` | scalar | The subroutine integrates from T to TOUT. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | rank 1; dimensions (15) | INFO(*) -- The basic task of the code is to integrate the differential equations from T to TOUT and return an answer at TOUT. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RTOL` | input | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | RTOL, ATOL -- These quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ATOL` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | RTOL, ATOL -- These quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IDID` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IDID -- This scalar quantity is an indicator reporting what the code did. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RWORK` | workspace | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | RWORK(*), LRW -- RWORK(*) is a REAL work array of length LRW which provides the code with needed storage space. | RWORK(*), LRW -- RWORK(*) is a REAL work array of length LRW which provides the code with needed storage space. Leading dimension: not established Workspace: RWORK(*), LRW -- RWORK(*) is a REAL work array of length LRW which provides the code with needed storage space. | required; null is not permitted for an ordinary Fortran actual argument |
| `LRW` | workspace | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | RWORK(*), LRW -- RWORK(*) is a REAL work array of length LRW which provides the code with needed storage space. | RWORK(*), LRW -- RWORK(*) is a REAL work array of length LRW which provides the code with needed storage space. Leading dimension: not established Workspace: RWORK(*), LRW -- RWORK(*) is a REAL work array of length LRW which provides the code with needed storage space. | required; null is not permitted for an ordinary Fortran actual argument |
| `IWORK` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | IWORK(*), LIW -- IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. | IWORK(*), LIW -- IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. Leading dimension: not established Workspace: IWORK(*), LIW -- IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. | required; null is not permitted for an ordinary Fortran actual argument |
| `LIW` | workspace | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IWORK(*), LIW -- IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. | IWORK(*), LIW -- IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. Leading dimension: not established Workspace: IWORK(*), LIW -- IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. | required; null is not permitted for an ordinary Fortran actual argument |
| `RPAR` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | RPAR, IPAR -- These are REAL and INTEGER parameter arrays which you can use for communication between your calling program and the F subroutine (and the JAC subroutine). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPAR` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | RPAR, IPAR -- These are REAL and INTEGER parameter arrays which you can use for communication between your calling program and the F subroutine (and the JAC subroutine). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JAC` | callback | `INTEGER` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | RPAR, IPAR -- These are REAL and INTEGER parameter arrays which you can use for communication between your calling program and the F subroutine (and the JAC subroutine). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
