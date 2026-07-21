# Purpose

I. PARAMETERS ..................................................... The user should use parameter names in the call sequence of SDRIV2 for those quantities whose value may be altered by SDRIV2. The parameters in the call sequence are: N = (Input) The number of differential equations. T = The independent variable. On input for the first call, T is the initial point. On output, T is the point at which the solution is given. Y = The vector of dependent variables. Y is used as input on the first call, to set the initial values. On output, Y is the computed solution vector. This array Y is passed in the call sequence of the user-provided routines F and G. Thus parameters required by F and G can be stored in this array in components N+1 and above. (Note: Changes by the user to the first N components of this array will take effect only after a restart, i.e., after setting MSTATE to +1(-1).) F = A subroutine supplied by the user. The name must be declared EXTERNAL in the user's calling program. This subroutine is of the form: SUBROUTINE F (N, T, Y, YDOT) REAL Y(*), YDOT(*) . . YDOT(1) = ... . . YDOT(N) = ...

# Description

This canonical unsafe binding exposes original SLATEC routine `SDRIV2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SDRIV2](https://www.netlib.org/slatec/src/sdriv2.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input positive equation count; it remains fixed for a continuation problem.

## `T`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Mutable independent variable: initial point on the first call and returned solution/root point thereafter.

## `Y`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Mutable length-at-least-`N` solution vector shared with the RHS and optional root callbacks.

## `F`

**Direction:** `callback`. **Fortran type:** `UNKNOWN`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Required synchronous RHS subroutine callback `F(N,T,Y,YDOT)`. `Y` is readable, `YDOT` is writable for `N` elements, callback-local `N=0` requests a controlled stop, and the callback has no user-data pointer or unwind permission. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `TOUT`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input requested output point for the current continuation call.

## `MSTATE`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input/output continuation state. Initialize to `+1` or `-1`; normal completion is `2`, root detection is `5`, controlled callback stops are `6`/`7`, and other documented states require recovery or continuation.

## `NROOT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input number of real root equations. Zero disables root search; otherwise `G` is invoked and the reported root index is stored in `IWORK(6)` using Fortran indexing.

## `EPS`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input/output relative accuracy request; the routine may raise a too-small value.

## `EWT`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input error-weight scale used to form `max(abs(Y(I)), EWT)` for the selected error control.

## `MINT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input method selector: `1` Adams, `2` Gear, or `3` automatic selection. It must not change without restarting.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

Mutable persistent real workspace. Its minimum is `16*N + 2*NROOT + 250` for `MINT=1`, `N*N + 10*N + 2*NROOT + 250` for `2`, or `N*N + 17*N + 2*NROOT + 250` for `3`.

## `LENW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input declared `WORK` length meeting the selected `MINT` formula.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

Mutable persistent integer workspace, at least `50` elements for `MINT=1` or `N+50` for `MINT=2/3`.

## `LENIW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input declared `IWORK` length meeting the selected `MINT` formula.

## `G`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Synchronous real root-function callback `G(N,T,Y,IROOT)`. It is used only when `NROOT` is nonzero, reads `Y\[0..N\]`, returns directly, may request a controlled stop through callback-local `N=0`, has no user-data pointer, and must not unwind. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `IERFLG`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input/output diagnostic status corresponding to source-documented warnings and recoverable setup or continuation failures.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Workspace and array requirements

- `Y`: not a workspace argument
- `WORK`: Mutable persistent real workspace. Its minimum is `16*N + 2*NROOT + 250` for `MINT=1`, `N*N + 10*N + 2*NROOT + 250` for `2`, or `N*N + 17*N + 2*NROOT + 250` for `3`.
- `IWORK`: Mutable persistent integer workspace, at least `50` elements for `MINT=1` or `N+50` for `MINT=2/3`.

# ABI notes

- Canonical Rust path: `slatec_sys::ode::sdriv2`
- Original SLATEC routine: `SDRIV2`
- Native symbol: `sdriv2_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [SDRIV2](https://www.netlib.org/slatec/src/sdriv2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
