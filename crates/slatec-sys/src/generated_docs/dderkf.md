# Purpose

This is the Runge-Kutta code in the package of differential equation solvers DEPAC, consisting of the codes DDERKF, DDEABM, and DDEBDF. Design of the package was by L. F. Shampine and H. A. Watts. It is documented in SAND-79-2374 , DEPAC - Design of a User Oriented Package of ODE Solvers. DDERKF is a driver for a modification of the code RKF45 written by H. A. Watts and L. F. Shampine Sandia Laboratories Albuquerque, New Mexico 87185 DDEPAC PACKAGE OVERVIEW ** You have a choice of three differential equation solvers from DDEPAC. The following brief descriptions are meant to aid you in choosing the most appropriate code for your problem. DDERKF is a fifth order Runge-Kutta code. It is the simplest of the three choices, both algorithmically and in the use of the code. DDERKF is primarily designed to solve non-stiff and mild- ly stiff differential equations when derivative evaluations are not expensive. It should generally not be used to get high accuracy results nor answers at a great many specific points. Because DDERKF has very low overhead costs, it will usually result in the least expensive integration when solving problems requiring a modest amount of accuracy and having equations that are not costly to evaluate. DDERKF attempts to discover when it is not suitable for the task posed. DDEABM is a variable order (one through twelve) Adams code. Its complexity lies somewhere between that of DDERKF and DDEBDF. DDEABM is primarily designed to solve non-stiff and mildly expensive, high accuracy results are needed or answers at many specific points are required. DDEABM attempts to discover DDEBDF is a variable order (one through five) backward differentiation formula code. It is the most complicated of the three choices. DDEBDF is primarily designed to solve stiff differential equations at crude to moderate tolerances. If the problem is very stiff at all, DDERKF and DDEABM will be quite inefficient compared to DDEBDF. However, DDEBDF will be inefficient compared to DDERKF and DDEABM on non-stiff problems because it uses much more storage, has a much larger overhead, and the low order formulas will not give high accuracies efficiently. The concept of stiffness cannot be described in a few words. If you do not know the problem to be stiff, try either DDERKF or DDEABM. Both of these codes will inform you of stiffness when the cost of solving such problems becomes important. Subroutine DDERKF uses a Runge-Kutta-Fehlberg (4,5) method to integrate a system of NEQ first order ordinary differential equations of the form DU/DX = DF(X,U) when the vector Y(*) of initial values for U(*) at X=T is given. The subroutine integrates from T to TOUT. It is easy to continue the integration to get results at additional TOUT. This is the interval mode of operation. It is also easy for the routine to return with the solution at each intermediate step on the way to TOUT. This is the intermediate-output mode of operation. DDERKF uses subprograms DRKFS, DFEHL, DHSTRT, DHVNRM, D1MACH, and the error handling routine XERMSG. The only machine dependent parameters to be assigned appear in D1MACH. DESCRIPTION OF THE ARGUMENTS TO DDERKF (AN OVERVIEW) ** The Parameters are:

# Description

This canonical unsafe binding exposes original SLATEC routine `DDERKF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DDERKF](https://www.netlib.org/slatec/src/dderkf.f).

# Arguments

## `DF`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

This is the name of a subroutine which you provide to define the differential equations. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `NEQ`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

This is the number of (first order) differential equations to be integrated.

## `T`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

This is a DOUBLE PRECISION value of the independent variable.

## `Y`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

This DOUBLE PRECISION array contains the solution components at T.

## `TOUT`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

This is a DOUBLE PRECISION point at which a solution is desired.

## `INFO`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (15).

The basic task of the code is to integrate the differential equations from T to TOUT and return an answer at TOUT. INFO(*) is an INTEGER array which is used to communicate exactly how you want this task to be carried out.

## `RTOL`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

These DOUBLE PRECISION quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. You may choose them to be both scalars or else both vectors.

## `ATOL`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

These DOUBLE PRECISION quantities represent relative and absolute error tolerances which you provide to indicate how accurately you wish the solution to be computed. You may choose them to be both scalars or else both vectors.

## `IDID`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

This scalar quantity is an indicator reporting what the code did. You must monitor this INTEGER variable to decide what action to take next.

## `RWORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

RWORK(*) is a DOUBLE PRECISION work array of length LRW which provides the code with needed storage space.

## `LRW`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

RWORK(*) is a DOUBLE PRECISION work array of length LRW which provides the code with needed storage space.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag.

## `LIW`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag.

## `RPAR`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

These are DOUBLE PRECISION and INTEGER parameter arrays which you can use for communication between your calling program and the DF subroutine.

## `IPAR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

These are DOUBLE PRECISION and INTEGER parameter arrays which you can use for communication between your calling program and the DF subroutine.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Workspace and array requirements

- `Y`: not a workspace argument
- `INFO`: not a workspace argument
- `RTOL`: not a workspace argument
- `ATOL`: not a workspace argument
- `RWORK`: RWORK(*) is a DOUBLE PRECISION work array of length LRW which provides the code with needed storage space.
- `IWORK`: IWORK(*) is an INTEGER work array of length LIW which provides the code with needed storage space and an across call flag.
- `RPAR`: not a workspace argument
- `IPAR`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::ode::callbacks::dderkf`
- Original SLATEC routine: `DDERKF`
- Native symbol: `dderkf_`
- ABI fingerprint: `subroutine:void(sub:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_i32),mut_i32,mut_f64,mut_f64_ptr_rank1,mut_f64,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [DDERKF](https://www.netlib.org/slatec/src/dderkf.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
