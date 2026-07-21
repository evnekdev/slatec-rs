# DDERKF

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve an initial value problem in ordinary differential equations using a Runge-Kutta-Fehlberg scheme.

## Description

This is the Runge-Kutta code in the package of differential equation solvers DEPAC, consisting of the codes DDERKF, DDEABM, and DDEBDF. Design of the package was by L. F. Shampine and H. A. Watts. It is documented in SAND-79-2374 , DEPAC - Design of a User Oriented Package of ODE Solvers. DDERKF is a driver for a modification of the code RKF45 written by H. A. Watts and L. F. Shampine Sandia Laboratories Albuquerque, New Mexico 87185 DDEPAC PACKAGE OVERVIEW ** You have a choice of three differential equation solvers from DDEPAC. The following brief descriptions are meant to aid you in choosing the most appropriate code for your problem. DDERKF is a fifth order Runge-Kutta code. It is the simplest of the three choices, both algorithmically and in the use of the code. DDERKF is primarily designed to solve non-stiff and mild- ly stiff differential equations when derivative evaluations are not expensive. It should generally not be used to get high accuracy results nor answers at a great many specific points. Because DDERKF has very low overhead costs, it will usually result in the least expensive integration when solving problems requiring a modest amount of accuracy and having equations that are not costly to evaluate. DDERKF attempts to discover when it is not suitable for the task posed. DDEABM is a variable order (one through twelve) Adams code. Its complexity lies somewhere between that of DDERKF and DDEBDF. DDEABM is primarily designed to solve non-stiff and mildly expensive, high accuracy results are needed or answers at many specific points are required. DDEABM attempts to discover DDEBDF is a variable order (one through five) backward differentiation formula code. It is the most complicated of the three choices. DDEBDF is primarily designed to solve stiff differential equations at crude to moderate tolerances. If the problem is very stiff at all, DDERKF and DDEABM will be quite inefficient compared to DDEBDF. However, DDEBDF will be inefficient compared to DDERKF and DDEABM on non-stiff problems because it uses much more storage, has a much larger overhead, and the low order formulas will not give high accuracies efficiently. The concept of stiffness cannot be described in a few words. If you do not know the problem to be stiff, try either DDERKF or DDEABM. Both of these codes will inform you of stiffness when the cost of solving such problems becomes important. Subroutine DDERKF uses a Runge-Kutta-Fehlberg (4,5) method to integrate a system of NEQ first order ordinary differential equations of the form DU/DX = DF(X,U) when the vector Y(*) of initial values for U(*) at X=T is given. The subroutine integrates from T to TOUT. It is easy to continue the integration to get results at additional TOUT. This is the interval mode of operation. It is also easy for the routine to return with the solution at each intermediate step on the way to TOUT. This is the intermediate-output mode of operation. DDERKF uses subprograms DRKFS, DFEHL, DHSTRT, DHVNRM, D1MACH, and the error handling routine XERMSG. The only machine dependent parameters to be assigned appear in D1MACH. DESCRIPTION OF THE ARGUMENTS TO DDERKF (AN OVERVIEW) ** The Parameters are:

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
- GAMS classifications: `I1A1A`
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

- Canonical provider: `main-src/src/dderkf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dderkf.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dderkf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dderkf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DDERKF](https://www.netlib.org/slatec/src/dderkf.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `DF` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | This is the name of a subroutine which you provide to define the differential equations. |
| 2 | `NEQ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | This is the number of (first order) differential equations to be integrated. |
| 3 | `T` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | This is a DOUBLE PRECISION value of the independent variable. |
| 4 | `Y` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | This DOUBLE PRECISION array contains the solution components at T. |
| 5 | `TOUT` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | This is a DOUBLE PRECISION point at which a solution is desired. |
| 6 | `INFO` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (15) | The basic task of the code is to integrate the differential equations from T to TOUT and return an answer at TOUT. INFO(*) is an INTEGER array which is used to communicate exactly how you want this task to be carried out. |
| 7 | `RTOL` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | These DOUBLE PRECISION quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. You may choose them to be both scalars or else both vectors. |
| 8 | `ATOL` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | These DOUBLE PRECISION quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. You may choose them to be both scalars or else both vectors. |
| 9 | `IDID` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | This scalar quantity is an indicator reporting what the code did. You must monitor this INTEGER variable to decide what action to take next. |
| 10 | `RWORK` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | RWORK(*) is a DOUBLE PRECISION work array of length LRW which provides the code with needed storage space. |
| 11 | `LRW` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | RWORK(*) is a DOUBLE PRECISION work array of length LRW which provides the code with needed storage space. |
| 12 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. |
| 13 | `LIW` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. |
| 14 | `RPAR` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | These are DOUBLE PRECISION and INTEGER parameter arrays which you can use for communication between your calling program and the DF subroutine. |
| 15 | `IPAR` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | These are DOUBLE PRECISION and INTEGER parameter arrays which you can use for communication between your calling program and the DF subroutine. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Storage and workspace requirements

`RWORK`: RWORK(*) is a DOUBLE PRECISION work array of length LRW which provides the code with needed storage space.

`IWORK`: IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::ode::callbacks::dderkf`. Native symbol: `dderkf_`. Declaration feature: `ode-callbacks`. Provider feature: `ode`. ABI fingerprint: `subroutine:void(sub:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_i32),mut_i32,mut_f64,mut_f64_ptr_rank1,mut_f64,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::ode::callbacks::dderkf`
- Public declaration feature: `ode-callbacks`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
