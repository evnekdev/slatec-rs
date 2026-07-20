# DHSTRT

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DDEABM, DDEBDF and DDERKF

## Description

DHSTRT computes a starting step size to be used in solving initial value problems in ordinary differential equations. ********************************************************************** Subroutine DHSTRT computes a starting step size to be used by an initial value method in solving ordinary differential equations. It is based on an estimate of the local Lipschitz constant for the differential equation (lower bound on a norm of the Jacobian) , a bound on the differential equation (first derivative) , and a bound on the partial derivative of the equation with respect to the independent variable. (all approximated near the initial point A) Subroutine DHSTRT uses a function subprogram DHVNRM for computing a vector norm. The maximum norm is presently utilized though it can easily be replaced by any other vector norm. It is presumed that any replacement norm routine would be carefully coded to prevent unnecessary underflows or overflows from occurring, and also, would not alter the vector or number of components. ********************************************************************** On input you must provide the following DF -- This is a subroutine of the form DF(X,U,UPRIME,RPAR,IPAR) which defines the system of first order differential equations to be solved. For the given values of X and the vector U(*)=(U(1),U(2),...,U(NEQ)) , the subroutine must evaluate the NEQ components of the system of differential equations DU/DX=DF(X,U) and store the derivatives in the array UPRIME(*), that is, UPRIME(I) = * DU(I)/DX * for equations I=1,...,NEQ. Subroutine DF must not alter X or U(*). You must declare the name DF in an external statement in your program that calls DHSTRT. You must dimension U and UPRIME in DF. RPAR and IPAR are DOUBLE PRECISION and INTEGER parameter arrays which you can use for communication between your program and subroutine DF. They are not used or altered by DHSTRT. If you do not need RPAR or IPAR, ignore these parameters by treating them as dummy arguments. If you do choose to use them, dimension them in your program and in DF as arrays of appropriate length. NEQ -- This is the number of (first order) differential equations to be integrated. A -- This is the initial point of integration. B -- This is a value of the independent variable used to define the direction of integration. A reasonable choice is to set B to the first point at which a solution is desired. You can also use B, if necessary, to restrict the length of the first integration step because the algorithm will not compute a starting step length which is bigger than ABS(B-A), unless B has been chosen too close to A. (it is presumed that DHSTRT has been called with B different from A on the machine being used. Also see the discussion about the parameter SMALL.) Y(*) -- This is the vector of initial values of the NEQ solution components at the initial point A. YPRIME(*) -- This is the vector of derivatives of the NEQ solution components at the initial point A. (defined by the differential equations in subroutine DF) ETOL -- This is the vector of error tolerances corresponding to the NEQ solution components. It is assumed that all elements are positive. Following the first integration step, the tolerances are expected to be used by the integrator in an error test which roughly requires that ABS(LOCAL ERROR) .LE. ETOL for each vector component. MORDER -- This is the order of the formula which will be used by the initial value method for taking the first integration step. SMALL -- This is a small positive machine dependent constant which is used for protecting against computations with numbers which are too small relative to the precision of floating point arithmetic. SMALL should be set to (approximately) the smallest positive DOUBLE PRECISION number such that (1.+SMALL) .GT. 1. on the machine being used. The quantity SMALL**(3/8) is used in computing increments of variables for approximating derivatives by differences. Also the algorithm will not compute a starting step length which is smaller than 100*SMALL*ABS(A). BIG -- This is a large positive machine dependent constant which is used for preventing machine overflows. A reasonable choice is to set big to (approximately) the square root of the largest DOUBLE PRECISION number which can be held in the machine. SPY(*),PV(*),YP(*),SF(*) -- These are DOUBLE PRECISION work arrays of length NEQ which provide the routine with needed storage space. RPAR,IPAR -- These are parameter arrays, of DOUBLE PRECISION and INTEGER type, respectively, which can be used for communication between your program and the DF subroutine. They are not used or altered by DHSTRT. ********************************************************************** On Output (after the return from DHSTRT), H -- is an appropriate starting step size to be attempted by the differential equation method. All parameters in the call list remain unchanged except for the working arrays SPY(*),PV(*),YP(*), and SF(*). **********************************************************************

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DDEABM, DDEBDF, DDERKF`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dhstrt.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dhstrt.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dhstrt.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dhstrt.f) — `verified_cached`
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
| `DF` | callback | `REAL` (`implicit_rule`) | `reviewed unsafe extern callback function pointer` | scalar | ********************************************************************** On input you must provide the following DF -- This is a subroutine of the form DF(X,U,UPRIME,RPAR,IPAR) which defines the system of first order differential equations to be solved. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEQ` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | For the given values of X and the vector U(*)=(U(1),U(2),...,U(NEQ)) , the subroutine must evaluate the NEQ components of the system of differential equations DU/DX=DF(X,U) and store the derivatives in the array UPRIME(*), that is, UPRIME(I) = * DU(I)/DX * for equations I=1,...,NEQ. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | DHSTRT computes a starting step size to be used in solving initial value problems in ordinary differential equations. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | B -- This is a value of the independent variable used to define the direction of integration. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Also see the discussion about the parameter SMALL.) Y(*) -- This is the vector of initial values of the NEQ solution components at the initial point A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YPRIME` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | YPRIME(*) -- This is the vector of derivatives of the NEQ solution components at the initial point A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ETOL` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | (defined by the differential equations in subroutine DF) ETOL -- This is the vector of error tolerances corresponding to the NEQ solution components. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MORDER` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | MORDER -- This is the order of the formula which will be used by the initial value method for taking the first integration step. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SMALL` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Also see the discussion about the parameter SMALL.) Y(*) -- This is the vector of initial values of the NEQ solution components at the initial point A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BIG` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | BIG -- This is a large positive machine dependent constant which is used for preventing machine overflows. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SPY` | workspace | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SPY(*),PV(*),YP(*),SF(*) -- These are DOUBLE PRECISION work arrays of length NEQ which provide the routine with needed storage space. | SPY(*),PV(*),YP(*),SF(*) -- These are DOUBLE PRECISION work arrays of length NEQ which provide the routine with needed storage space. Leading dimension: not established Workspace: SPY(*),PV(*),YP(*),SF(*) -- These are DOUBLE PRECISION work arrays of length NEQ which provide the routine with needed storage space. | required; null is not permitted for an ordinary Fortran actual argument |
| `PV` | workspace | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SPY(*),PV(*),YP(*),SF(*) -- These are DOUBLE PRECISION work arrays of length NEQ which provide the routine with needed storage space. | SPY(*),PV(*),YP(*),SF(*) -- These are DOUBLE PRECISION work arrays of length NEQ which provide the routine with needed storage space. Leading dimension: not established Workspace: SPY(*),PV(*),YP(*),SF(*) -- These are DOUBLE PRECISION work arrays of length NEQ which provide the routine with needed storage space. | required; null is not permitted for an ordinary Fortran actual argument |
| `YP` | workspace | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SPY(*),PV(*),YP(*),SF(*) -- These are DOUBLE PRECISION work arrays of length NEQ which provide the routine with needed storage space. | SPY(*),PV(*),YP(*),SF(*) -- These are DOUBLE PRECISION work arrays of length NEQ which provide the routine with needed storage space. Leading dimension: not established Workspace: SPY(*),PV(*),YP(*),SF(*) -- These are DOUBLE PRECISION work arrays of length NEQ which provide the routine with needed storage space. | required; null is not permitted for an ordinary Fortran actual argument |
| `SF` | workspace | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SPY(*),PV(*),YP(*),SF(*) -- These are DOUBLE PRECISION work arrays of length NEQ which provide the routine with needed storage space. | SPY(*),PV(*),YP(*),SF(*) -- These are DOUBLE PRECISION work arrays of length NEQ which provide the routine with needed storage space. Leading dimension: not established Workspace: SPY(*),PV(*),YP(*),SF(*) -- These are DOUBLE PRECISION work arrays of length NEQ which provide the routine with needed storage space. | required; null is not permitted for an ordinary Fortran actual argument |
| `RPAR` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | ********************************************************************** On input you must provide the following DF -- This is a subroutine of the form DF(X,U,UPRIME,RPAR,IPAR) which defines the system of first order differential equations to be solved. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPAR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | ********************************************************************** On input you must provide the following DF -- This is a subroutine of the form DF(X,U,UPRIME,RPAR,IPAR) which defines the system of first order differential equations to be solved. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `H` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | ********************************************************************** On Output (after the return from DHSTRT), H -- is an appropriate starting step size to be attempted by the differential equation method. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
