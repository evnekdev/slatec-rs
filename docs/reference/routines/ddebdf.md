# DDEBDF

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve an initial value problem in ordinary differential equations using backward differentiation formulas. It is intended primarily for stiff problems.

## Description

This is the backward differentiation code in the package of differential equation solvers DEPAC, consisting of the codes DDERKF, DDEABM, and DDEBDF. Design of the package was by L. F. Shampine and H. A. Watts. It is documented in SAND-79-2374 , DEPAC - Design of a User Oriented Package of ODE Solvers. DDEBDF is a driver for a modification of the code LSODE written by A. C. Hindmarsh Lawrence Livermore Laboratory Livermore, California 94550 ********************************************************************** ** DEPAC PACKAGE OVERVIEW ** ********************************************************************** You have a choice of three differential equation solvers from DEPAC. The following brief descriptions are meant to aid you in choosing the most appropriate code for your problem. DDERKF is a fifth order Runge-Kutta code. It is the simplest of the three choices, both algorithmically and in the use of the code. DDERKF is primarily designed to solve non-stiff and mildly stiff differential equations when derivative evaluations are not expensive. It should generally not be used to get high accuracy results nor answers at a great many specific points. Because DDERKF has very low overhead costs, it will usually result in the least expensive integration when solving problems requiring a modest amount of accuracy and having equations that are not costly to evaluate. DDERKF attempts to discover when it is not suitable for the task posed. DDEABM is a variable order (one through twelve) Adams code. Its complexity lies somewhere between that of DDERKF and DDEBDF. DDEABM is primarily designed to solve non-stiff and mildly stiff differential equations when derivative evaluations are expensive, high accuracy results are needed or answers at many specific points are required. DDEABM attempts to discover when it is not suitable for the task posed. DDEBDF is a variable order (one through five) backward differentiation formula code. It is the most complicated of the three choices. DDEBDF is primarily designed to solve stiff differential equations at crude to moderate tolerances. If the problem is very stiff at all, DDERKF and DDEABM will be quite inefficient compared to DDEBDF. However, DDEBDF will be inefficient compared to DDERKF and DDEABM on non-stiff problems because it uses much more storage, has a much larger overhead, and the low order formulas will not give high accuracies efficiently. The concept of stiffness cannot be described in a few words. If you do not know the problem to be stiff, try either DDERKF or DDEABM. Both of these codes will inform you of stiffness when the cost of solving such problems becomes important. ********************************************************************** ** ABSTRACT ** ********************************************************************** Subroutine DDEBDF uses the backward differentiation formulas of orders one through five to integrate a system of NEQ first order ordinary differential equations of the form DU/DX = DF(X,U) when the vector Y(*) of initial values for U(*) at X=T is given. The subroutine integrates from T to TOUT. It is easy to continue the integration to get results at additional TOUT. This is the interval mode of operation. It is also easy for the routine to return with the solution at each intermediate step on the way to TOUT. This is the intermediate-output mode of operation. ********************************************************************** * Description of The Arguments To DDEBDF (An Overview) * ********************************************************************** The Parameters are: DF -- This is the name of a subroutine which you provide to define the differential equations. NEQ -- This is the number of (first order) differential equations to be integrated. T -- This is a DOUBLE PRECISION value of the independent variable. Y(*) -- This DOUBLE PRECISION array contains the solution components at T. TOUT -- This is a DOUBLE PRECISION point at which a solution is desired. INFO(*) -- The basic task of the code is to integrate the differential equations from T to TOUT and return an answer at TOUT. INFO(*) is an INTEGER array which is used to communicate exactly how you want this task to be carried out. RTOL, ATOL -- These DOUBLE PRECISION quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. You may choose them to be both scalars or else both vectors. IDID -- This scalar quantity is an indicator reporting what the code did. You must monitor this INTEGER variable to decide what action to take next. RWORK(*), LRW -- RWORK(*) is a DOUBLE PRECISION work array of length LRW which provides the code with needed storage space. IWORK(*), LIW -- IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. RPAR, IPAR -- These are DOUBLE PRECISION and INTEGER parameter arrays which you can use for communication between your calling program and the DF subroutine (and the DJAC subroutine). DJAC -- This is the name of a subroutine which you may choose to provide for defining the Jacobian matrix of partial derivatives DF/DU. Quantities which are used as input items are NEQ, T, Y(*), TOUT, INFO(*), RTOL, ATOL, RWORK(1), LRW, IWORK(1), IWORK(2), and LIW. Quantities which may be altered by the code are T, Y(*), INFO(1), RTOL, ATOL, IDID, RWORK(*) and IWORK(*). ********************************************************************** * INPUT -- What To Do On The First Call To DDEBDF * ********************************************************************** The first call of the code is defined to be the start of each new problem. Read through the descriptions of all the following items, provide sufficient storage space for designated arrays, set appropriate variables for the initialization of the problem, and give information about how you want the problem to be solved. DF -- Provide a subroutine of the form DF(X,U,UPRIME,RPAR,IPAR) to define the system of first order differential equations which is to be solved. For the given values of X and the vector U(*)=(U(1),U(2),...,U(NEQ)) , the subroutine must evaluate the NEQ components of the system of differential equations DU/DX=DF(X,U) and store the derivatives in the array UPRIME(*), that is, UPRIME(I) = * DU(I)/DX * for equations I=1,...,NEQ. Subroutine DF must not alter X or U(*). You must declare the name DF in an external statement in your program that calls DDEBDF. You must dimension U and UPRIME in DF. RPAR and IPAR are DOUBLE PRECISION and INTEGER parameter arrays which you can use for communication between your calling program and subroutine DF. They are not used or altered by DDEBDF. If you do not need RPAR or IPAR, ignore these parameters by treating them as dummy arguments. If you do choose to use them, dimension them in your calling program and in DF as arrays of appropriate length. NEQ -- Set it to the number of differential equations. (NEQ .GE. 1) T -- Set it to the initial point of the integration. You must use a program variable for T because the code changes its value. Y(*) -- Set this vector to the initial values of the NEQ solution components at the initial point. You must dimension Y at least NEQ in your calling program. TOUT -- Set it to the first point at which a solution is desired. You can take TOUT = T, in which case the code will evaluate the derivative of the solution at T and return. Integration either forward in T (TOUT .GT. T) or backward in T (TOUT .LT. T) is permitted. The code advances the solution from T to TOUT using step sizes which are automatically selected so as to achieve the desired accuracy. If you wish, the code will return with the solution and its derivative following each intermediate step (intermediate-output mode) so that you can monitor them, but you still must provide TOUT in accord with the basic aim of the code. The first step taken by the code is a critical one because it must reflect how fast the solution changes near the initial point. The code automatically selects an initial step size which is practically always suitable for the problem. By using the fact that the code will not step past TOUT in the first step, you could, if necessary, restrict the length of the initial step size. For some problems it may not be permissible to integrate past a point TSTOP because a discontinuity occurs there or the solution or its derivative is not defined beyond TSTOP. When you have declared a TSTOP point (see INFO(4) and RWORK(1)), you have told the code not to integrate past TSTOP. In this case any TOUT beyond TSTOP is invalid

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

- Canonical provider: `main-src/src/ddebdf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ddebdf.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ddebdf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ddebdf.f) — `verified_cached`
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
- Public declaration feature: `raw-ffi-callbacks`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
