# Purpose

Version 92.1 I. CHOOSING THE CORRECT ROUTINE ................................... SDRIV DDRIV CDRIV These are the generic names for three packages for solving initial value problems for ordinary differential equations. SDRIV uses single precision arithmetic. DDRIV uses double precision arithmetic. CDRIV allows complex-valued differential equations, integrated with respect to a single, real, independent variable. As an aid in selecting the proper program, the following is a discussion of the important options or restrictions associated with each program: A. CDRIV1 should be tried first for those routine problems with no more than 200 differential equations (CDRIV2 and CDRIV3 have no such restriction.) Internally this routine has two important technical defaults: 1. Numerical approximation of the Jacobian matrix of the right hand side is used. 2. The stiff solver option is used. Most users of CDRIV1 should not have to concern themselves with these details. B. CDRIV2 should be considered for those problems for which CDRIV1 is inadequate. For example, CDRIV1 may have difficulty with problems having zero initial conditions and zero derivatives. In this case CDRIV2, with an appropriate value of the parameter EWT, should perform more efficiently. CDRIV2 provides three important additional options: 1. The nonstiff equation solver (as well as the stiff solver) is available. 2. The root-finding option is available. 3. The program can dynamically select either the non-stiff or the stiff methods. Internally this routine also defaults to the numerical approximation of the Jacobian matrix of the right hand side. C. CDRIV3 is the most flexible, and hence the most complex, of the programs. Its important additional features include: 1. The ability to exploit band structure in the Jacobian matrix. 2. The ability to solve some implicit differential equations, i.e., those having the form: A(Y,T)*dY/dT = F(Y,T). 3. The option of integrating in the one step mode. 4. The option of allowing the user to provide a routine which computes the analytic Jacobian matrix of the right 5. The option of allowing the user to provide a routine which does all the matrix algebra associated with corrections to the solution components.

# Description

This canonical unsafe binding exposes original SLATEC routine `CDRIV1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CDRIV1](https://www.netlib.org/slatec/src/cdriv1.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input equation count, constrained to `1..=200` by this convenience driver.

## `T`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Mutable real independent variable.

## `Y`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

Mutable length-at-least-`N` `Complex32` solution vector in selected GNU Fortran complex layout.

## `F`

**Direction:** `callback`. **Fortran type:** `UNKNOWN`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Required synchronous complex RHS subroutine callback `F(N,T,Y,YDOT)`. `Y` is readable and `YDOT` writable for `N` complex values; it has no user-data pointer and must not unwind. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `TOUT`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input real output point for the current integration call.

## `MSTATE`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input/output continuation state with the same source-documented completion and recovery protocol as SDRIV1.

## `EPS`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input/output real relative accuracy request.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

Mutable persistent `Complex32` workspace with at least `N*N + 11*N + 300` elements.

## `LENW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input declared `WORK` length meeting the stated formula.

## `IERFLG`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input/output diagnostic status for the selected legacy driver.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Workspace and array requirements

- `Y`: not a workspace argument
- `WORK`: Mutable persistent `Complex32` workspace with at least `N*N + 11*N + 300` elements.

# ABI notes

- Canonical Rust path: `slatec_sys::ode::cdriv1`
- Original SLATEC routine: `CDRIV1`
- Native symbol: `cdriv1_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CDRIV1](https://www.netlib.org/slatec/src/cdriv1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
