# Purpose

I. PARAMETERS ..................................................... The user should use parameter names in the call sequence of CDRIV2 for those quantities whose value may be altered by CDRIV2. The parameters in the call sequence are: N = (Input) The number of differential equations. T = (Real) The independent variable. On input for the first call, T is the initial point. On output, T is the point at which the solution is given. Y = (Complex) The vector of dependent variables. Y is used as input on the first call, to set the initial values. On output, Y is the computed solution vector. This array Y is passed in the call sequence of the user-provided

# Description

This canonical unsafe binding exposes original SLATEC routine `CDRIV2`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CDRIV2](https://www.netlib.org/slatec/src/cdriv2.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input positive equation count.

## `T`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Mutable real independent variable.

## `Y`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

Mutable length-at-least-`N` `Complex32` solution vector shared with callbacks.

## `F`

**Direction:** `callback`. **Fortran type:** `UNKNOWN`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Required synchronous complex RHS subroutine callback `F(N,T,Y,YDOT)`. It has no user-data pointer and must not unwind. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `TOUT`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input real output point for the current continuation call.

## `MSTATE`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input/output continuation state; root and controlled-stop values follow the selected CDRIV2 prologue.

## `NROOT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input count of real root equations; zero disables `G` and otherwise records a one-based root index in `IWORK(6)`.

## `EPS`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input/output real relative accuracy request.

## `EWT`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

Input real error-weight scale used for complex solution error control.

## `MINT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input Adams (`1`), Gear (`2`), or automatic (`3`) method selector; restart before changing it.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

Mutable persistent `Complex32` workspace. Its minimum is `16*N + 2*NROOT + 250`, `N*N + 10*N + 2*NROOT + 250`, or `N*N + 17*N + 2*NROOT + 250` for `MINT=1`, `2`, or `3` respectively.

## `LENW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input declared complex workspace length.

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

Mutable persistent integer workspace with at least `50` elements for `MINT=1` or `N+50` otherwise.

## `LENIW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input declared integer workspace length.

## `G`

**Direction:** `callback`. **Fortran type:** `REAL`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Optional synchronous real root-function callback `G(N,T,Y,IROOT)` over the complex solution vector. It is used only when `NROOT` is nonzero, returns an `f32` directly, has no user-data pointer, and must not unwind. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `IERFLG`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input/output diagnostic status for warnings and recoverable failures.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Workspace and array requirements

- `Y`: not a workspace argument
- `WORK`: Mutable persistent `Complex32` workspace. Its minimum is `16*N + 2*NROOT + 250`, `N*N + 10*N + 2*NROOT + 250`, or `N*N + 17*N + 2*NROOT + 250` for `MINT=1`, `2`, or `3` respectively.
- `IWORK`: Mutable persistent integer workspace with at least `50` elements for `MINT=1` or `N+50` otherwise.

# ABI notes

- Canonical Rust path: `slatec_sys::ode::cdriv2`
- Original SLATEC routine: `CDRIV2`
- Native symbol: `cdriv2_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [CDRIV2](https://www.netlib.org/slatec/src/cdriv2.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
