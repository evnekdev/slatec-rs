# Purpose

This is the Adams code in the package of differential equation solvers DEPAC, consisting of the codes DDERKF, DDEABM, and DDEBDF. Design of the package was by L. F. Shampine and H. A. Watts. It is documented in SAND79-2374 , DEPAC - Design of a User Oriented Package of ODE Solvers. DDEABM is a driver for a modification of the code ODE written by L. F. Shampine and M. K. Gordon Sandia Laboratories Albuquerque, New Mexico 87185 Subroutine DDEABM uses the Adams-Bashforth-Moulton Predictor-Corrector formulas of orders one through twelve to integrate a system of NEQ first order ordinary differential equations of the form DU/DX = DF(X,U) when the vector Y(*) of initial values for U(*) at X=T is given. The subroutine integrates from T to TOUT. It is easy to continue the integration to get results at additional TOUT. This is the interval mode of operation. It is also easy for the routine to return with the solution at each intermediate step on the way to TOUT. This is the intermediate-output mode of operation. DDEABM uses subprograms DDES, DSTEPS, DINTP, DHSTRT, DHVNRM, D1MACH, and the error handling routine XERMSG. The only machine dependent parameters to be assigned appear in D1MACH. Description of The Arguments To DDEABM (An Overview) * The Parameters are

# Description

This canonical unsafe binding exposes original SLATEC routine `DDEABM`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DDEABM](https://www.netlib.org/slatec/src/ddeabm.f).

# Arguments

## `DF`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

This is the name of a subroutine which you provide to define the differential equations. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `NEQ`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

This is the number of (first order) differential equations to be integrated.

## `T`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

This is a DOUBLE PRECISION value of the independent variable. The solution was successfully advanced to the output value of T.

## `Y`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

This DOUBLE PRECISION array contains the solution components at T. Contains the computed solution approximation at T. You may also be interested in the approximate derivative of the solution at T. It is contained in is obtained by interpolation. Task Interrupted *** Reported by negative values of IDID.

## `TOUT`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

This is a DOUBLE PRECISION point at which a solution is desired. must be different from T. You cannot change the direction of integration without restarting. Following An Interrupted Task *** To show the code that you realize the task was interrupted and that you want to continue, you must take appropriate action and reset INFO(1) = 1 If.

## `INFO`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (15).

The basic task of the code is to integrate the differential equations from T to TOUT and return an answer at TOUT. INFO(*) is an INTEGER array which is used to communicate exactly how you want this task to be carried out. Use the INFO array to give the code more details about how you want your problem solved. This array should be dimensioned of length 15 to accommodate other members of DEPAC or possible future extensions, though DDEABM uses only the first four entries. You must respond to all of the following items which are arranged as questions. The simplest use of the code corresponds to answering all questions as YES ,i.

## `RTOL`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

These DOUBLE PRECISION quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. You may choose them to be both scalars or else both vectors. You must assign relative (RTOL) and absolute (ATOL) error tolerances to tell the code how accurately you want the solution to be computed. They must be defined as program variables because the code may change them. You have two choices -- Both RTOL and ATOL are scalars. (INFO(2)=0) Both RTOL and ATOL are vectors.

## `ATOL`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

These DOUBLE PRECISION quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. You may choose them to be both scalars or else both vectors. You must assign relative (RTOL) and absolute (ATOL) error tolerances to tell the code how accurately you want the solution to be computed. They must be defined as program variables because the code may change them. You have two choices -- Both RTOL and ATOL are scalars. (INFO(2)=0) Both RTOL and ATOL are vectors.

## `IDID`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

This scalar quantity is an indicator reporting what the code did. You must monitor this INTEGER variable to decide what action to take next. Reports what the code did Task Completed *** Reported by positive values of IDID 1 -- A step was successfully taken in the intermediate-output mode. The code has not yet reached TOUT. 2 -- The integration to TOUT was successfully completed (T=TOUT) by stepping exactly to TOUT. 3 -- The integration to TOUT was successfully completed (T=TOUT) by stepping past TOUT.

## `RWORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

RWORK(*) is a DOUBLE PRECISION work array of length LRW which provides the code with needed storage space. Dimension this DOUBLE PRECISION work array of length LRW in your calling program. If you have set INFO(4)=0, you can ignore this optional input parameter. Otherwise you must define a stopping point TSTOP by setting RWORK(1) = TSTOP. (for some problems it may not be permissible to integrate past a point TSTOP because a discontinuity occurs there or the solution or its derivative is not defined beyond TSTOP. ) Contain information which is usually of no interest to the user but necessary for subsequent calls.

## `LRW`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

RWORK(*) is a DOUBLE PRECISION work array of length LRW which provides the code with needed storage space. Set it to the declared length of the RWORK array. You must have LRW. GE. 130+21*NEQ.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. Dimension this INTEGER work array of length LIW in your calling program. Contain information which is usually of no interest to the user but necessary for subsequent calls. However, you may find use for.

## `LIW`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. Set it to the declared length of the IWORK array. You must have LIW. GE. 51.

## `RPAR`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

These are DOUBLE PRECISION and INTEGER parameter arrays which you can use for communication between your calling program and the DF subroutine. These are parameter arrays, of DOUBLE PRECISION and INTEGER type, respectively. You can use them for communication between your program that calls DDEABM and the DF subroutine. They are not used or altered by DDEABM. If you do not need RPAR or IPAR, ignore these parameters by treating them as dummy arguments. If you do choose to use them, dimension them in your calling program and in DF as arrays of appropriate length.

## `IPAR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

These are DOUBLE PRECISION and INTEGER parameter arrays which you can use for communication between your calling program and the DF subroutine. These are parameter arrays, of DOUBLE PRECISION and INTEGER type, respectively. You can use them for communication between your program that calls DDEABM and the DF subroutine. They are not used or altered by DDEABM. If you do not need RPAR or IPAR, ignore these parameters by treating them as dummy arguments. If you do choose to use them, dimension them in your calling program and in DF as arrays of appropriate length.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `INFO` | `0` | 0 NO -- not applicable here. See below for continuation calls. **** is specified by the error tolerances RTOL and ATOL. The simplest use is to take them both to be scalars. To obtain more flexibility, they can both be vectors. The code must be told your choice. Are both error tolerances RTOL, ATOL scalars ... and input scalars for both RTOL and ATOL |
| `INFO` | `1` | 1 and input arrays for both RTOL and ATOL **** of TOUT by steps. If you wish, it will return the computed solution and derivative at the next intermediate step (the intermediate-output mode) or TOUT, whichever comes first. This is a good way to proceed if you want to see the behavior of the solution. If you must have solutions at a great many specific TOUT points, this code will compute them efficiently. Do you want the solution only at TOUT (and not at the next intermediate step) ... |
| `INFO` | `0` | 0 |
| `INFO` | `1` | 1 **** values TOUT efficiently, this code may integrate past TOUT and interpolate to obtain the result at TOUT. Sometimes it is not possible to integrate beyond some point TSTOP because the equation changes there or it is not defined past TSTOP. Then you must tell the code not to go past. Can the integration be carried out without any Restrictions on the independent variable T ... |
| `INFO` | `1` | and define the stopping point TSTOP by setting RWORK(1)=TSTOP **** 1 if you wish to continue after an interrupted task. |
| `INFO` | `0` | 0 on a continuation call unless you want the code to restart at the current T. Following A Completed Task *** If |

# Workspace and array requirements

- `Y`: not a workspace argument
- `INFO`: not a workspace argument
- `RTOL`: not a workspace argument
- `ATOL`: not a workspace argument
- `RWORK`: RWORK(*) is a DOUBLE PRECISION work array of length LRW which provides the code with needed storage space. Dimension this DOUBLE PRECISION work array of length LRW in your calling program. If you have set INFO(4)=0, you can ignore this optional input parameter. Otherwise you must define a stopping point TSTOP by setting RWORK(1) = TSTOP. (for some problems it may not be permissible to integrate past a point TSTOP because a discontinuity occurs there or the solution or its derivative is not defined beyond TSTOP. ) Contain information which is usually of no interest to the user but necessary for subsequent calls.
- `IWORK`: IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag. Dimension this INTEGER work array of length LIW in your calling program. Contain information which is usually of no interest to the user but necessary for subsequent calls. However, you may find use for.
- `RPAR`: not a workspace argument
- `IPAR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::ode::callbacks::ddeabm`
- Original SLATEC routine: `DDEABM`
- Native symbol: `ddeabm_`
- ABI fingerprint: `subroutine:void(sub:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_i32),mut_i32,mut_f64,mut_f64_ptr_rank1,mut_f64,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [DDEABM](https://www.netlib.org/slatec/src/ddeabm.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
