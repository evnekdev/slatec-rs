# DDEABM

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve an initial value problem in ordinary differential equations using an Adams-Bashforth method.

## Description

This is the Adams code in the package of differential equation solvers DEPAC, consisting of the codes DDERKF, DDEABM, and DDEBDF. Design of the package was by L. F. Shampine and H. A. Watts. It is documented in SAND79-2374 , DEPAC - Design of a User Oriented Package of ODE Solvers. DDEABM is a driver for a modification of the code ODE written by L. F. Shampine and M. K. Gordon Sandia Laboratories Albuquerque, New Mexico 87185 Subroutine DDEABM uses the Adams-Bashforth-Moulton Predictor-Corrector formulas of orders one through twelve to integrate a system of NEQ first order ordinary differential equations of the form DU/DX = DF(X,U) when the vector Y(*) of initial values for U(*) at X=T is given. The subroutine integrates from T to TOUT. It is easy to continue the integration to get results at additional TOUT. This is the interval mode of operation. It is also easy for the routine to return with the solution at each intermediate step on the way to TOUT. This is the intermediate-output mode of operation. DDEABM uses subprograms DDES, DSTEPS, DINTP, DHSTRT, DHVNRM, D1MACH, and the error handling routine XERMSG. The only machine dependent parameters to be assigned appear in D1MACH. Description of The Arguments To DDEABM (An Overview) * The Parameters are

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
- GAMS classifications: `I1A1B`
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

- Canonical provider: `main-src/src/ddeabm.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ddeabm.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ddeabm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ddeabm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DDEABM](https://www.netlib.org/slatec/src/ddeabm.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `DF` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | This is the name of a subroutine which you provide to define the differential equations. |
| 2 | `NEQ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | This is the number of (first order) differential equations to be integrated. |
| 3 | `T` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | This is a DOUBLE PRECISION value of the independent variable. The solution was successfully advanced to the output value of T. |
| 4 | `Y` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | This DOUBLE PRECISION array contains the solution components at T. Contains the computed solution approximation at T. You may also be interested in the approximate derivative of the solution at T. It is contained in is obtained by interpolation. Task Interrupted *** Reported by negative values of IDID. |
| 5 | `TOUT` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | This is a DOUBLE PRECISION point at which a solution is desired. must be different from T. You cannot change the direction of integration without restarting. Following An Interrupted Task *** To show the code that you realize the task was interrupted and that you want to continue, you must take appropriate action and reset INFO(1) = 1 If. |
| 6 | `INFO` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (15) | The basic task of the code is to integrate the differential equations from T to TOUT and return an answer at TOUT. INFO(*) is an INTEGER array which is used to communicate exactly how you want this task to be carried out. Use the INFO array to give the code more details about how you want your problem solved. This array should be dimensioned of length 15 to accommodate other members of DEPAC or possible future extensions, though DDEABM uses only the first four entries. You must respond to all of the following items which are arranged as questions. The simplest use of the code corresponds to answering all questions as YES ,i. |
| 7 | `RTOL` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | These DOUBLE PRECISION quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. You may choose them to be both scalars or else both vectors. You must assign relative (RTOL) and absolute (ATOL) error tolerances to tell the code how accurately you want the solution to be computed. They must be defined as program variables because the code may change them. You have two choices -- Both RTOL and ATOL are scalars. (INFO(2)=0) Both RTOL and ATOL are vectors. |
| 8 | `ATOL` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | These DOUBLE PRECISION quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. You may choose them to be both scalars or else both vectors. You must assign relative (RTOL) and absolute (ATOL) error tolerances to tell the code how accurately you want the solution to be computed. They must be defined as program variables because the code may change them. You have two choices -- Both RTOL and ATOL are scalars. (INFO(2)=0) Both RTOL and ATOL are vectors. |
| 9 | `IDID` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | This scalar quantity is an indicator reporting what the code did. You must monitor this INTEGER variable to decide what action to take next. Reports what the code did Task Completed *** Reported by positive values of IDID 1 -- A step was successfully taken in the intermediate-output mode. The code has not yet reached TOUT. 2 -- The integration to TOUT was successfully completed (T=TOUT) by stepping exactly to TOUT. 3 -- The integration to TOUT was successfully completed (T=TOUT) by stepping past TOUT. |
| 10 | `RWORK` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | RWORK(*) is a DOUBLE PRECISION work array of length LRW which provides the code with needed storage space. Dimension this DOUBLE PRECISION work array of length LRW in your calling program. If you have set INFO(4)=0, you can ignore this optional input parameter. Otherwise you must define a stopping point TSTOP by setting RWORK(1) = TSTOP. (for some problems it may not be permissible to integrate past a point TSTOP because a discontinuity occurs there or the solution or its derivative is not defined beyond TSTOP. ) Contain information which is usually of no interest to the user but necessary for subsequent calls. |
| 11 | `LRW` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | RWORK(*) is a DOUBLE PRECISION work array of length LRW which provides the code with needed storage space. Set it to the declared length of the RWORK array. You must have LRW. GE. 130+21*NEQ. |
| 12 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. Dimension this INTEGER work array of length LIW in your calling program. Contain information which is usually of no interest to the user but necessary for subsequent calls. However, you may find use for. |
| 13 | `LIW` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. Set it to the declared length of the IWORK array. You must have LIW. GE. 51. |
| 14 | `RPAR` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | These are DOUBLE PRECISION and INTEGER parameter arrays which you can use for communication between your calling program and the DF subroutine. These are parameter arrays, of DOUBLE PRECISION and INTEGER type, respectively. You can use them for communication between your program that calls DDEABM and the DF subroutine. They are not used or altered by DDEABM. If you do not need RPAR or IPAR, ignore these parameters by treating them as dummy arguments. If you do choose to use them, dimension them in your calling program and in DF as arrays of appropriate length. |
| 15 | `IPAR` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | These are DOUBLE PRECISION and INTEGER parameter arrays which you can use for communication between your calling program and the DF subroutine. These are parameter arrays, of DOUBLE PRECISION and INTEGER type, respectively. You can use them for communication between your program that calls DDEABM and the DF subroutine. They are not used or altered by DDEABM. If you do not need RPAR or IPAR, ignore these parameters by treating them as dummy arguments. If you do choose to use them, dimension them in your calling program and in DF as arrays of appropriate length. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 NO -- not applicable here. See below for continuation calls. **** is specified by the error tolerances RTOL and ATOL. The simplest use is to take them both to be scalars. To obtain more flexibility, they can both be vectors. The code must be told your choice. Are both error tolerances RTOL, ATOL scalars ... and input scalars for both RTOL and ATOL |
| `INFO` | `1` | 1 and input arrays for both RTOL and ATOL **** of TOUT by steps. If you wish, it will return the computed solution and derivative at the next intermediate step (the intermediate-output mode) or TOUT, whichever comes first. This is a good way to proceed if you want to see the behavior of the solution. If you must have solutions at a great many specific TOUT points, this code will compute them efficiently. Do you want the solution only at TOUT (and not at the next intermediate step) ... |
| `INFO` | `0` | 0 |
| `INFO` | `1` | 1 **** values TOUT efficiently, this code may integrate past TOUT and interpolate to obtain the result at TOUT. Sometimes it is not possible to integrate beyond some point TSTOP because the equation changes there or it is not defined past TSTOP. Then you must tell the code not to go past. Can the integration be carried out without any Restrictions on the independent variable T ... |
| `INFO` | `1` | and define the stopping point TSTOP by setting RWORK(1)=TSTOP **** 1 if you wish to continue after an interrupted task. |
| `INFO` | `0` | 0 on a continuation call unless you want the code to restart at the current T. Following A Completed Task *** If |

### Storage and workspace requirements

`RWORK`: RWORK(*) is a DOUBLE PRECISION work array of length LRW which provides the code with needed storage space. Dimension this DOUBLE PRECISION work array of length LRW in your calling program. If you have set INFO(4)=0, you can ignore this optional input parameter. Otherwise you must define a stopping point TSTOP by setting RWORK(1) = TSTOP. (for some problems it may not be permissible to integrate past a point TSTOP because a discontinuity occurs there or the solution or its derivative is not defined beyond TSTOP. ) Contain information which is usually of no interest to the user but necessary for subsequent calls.

`IWORK`: IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. Dimension this INTEGER work array of length LIW in your calling program. Contain information which is usually of no interest to the user but necessary for subsequent calls. However, you may find use for.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::ode::callbacks::ddeabm`. Native symbol: `ddeabm_`. Declaration feature: `ode-callbacks`. Provider feature: `ode`. ABI fingerprint: `subroutine:void(sub:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_i32),mut_i32,mut_f64,mut_f64_ptr_rank1,mut_f64,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::ode::callbacks::ddeabm`
- Public declaration feature: `ode-callbacks`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
