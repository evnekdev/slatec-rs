# Purpose

Abstract **** a double precision routine **** DPFQAD computes the integral on (X1,X2) of a product of a function F and the ID-th derivative of a B-spline, using the PP-representation (C,XI,LXI,K). (X1,X2) is normally a sub- interval of XI(1) .LE. X .LE. XI(LXI+1). An integration routine, DPPGQ8 (a modification of GAUS8), integrates the product on subintervals of (X1,X2) formed by the included break points. Integration outside of (XI(1),XI(LXI+1)) is permitted provided F is defined. The maximum number of significant digits obtainable in DBSQAD is the smaller of 18 and the number of digits carried in double precision arithmetic.

# Description

This canonical unsafe binding exposes original SLATEC routine `DPFQAD`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DPFQAD](https://www.netlib.org/slatec/src/dpfqad.f).

# Arguments

## `F`

**Direction:** `callback`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

Required synchronous double-precision scalar integrand callback `F(X)`. It receives only a readable scalar pointer, returns directly, has no user-data pointer, must not retain the pointer, and must not unwind through Fortran. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `LDC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input leading dimension of the Fortran column-major coefficient matrix `C`; it must be at least `K`.

## `C`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 2; dimensions (LDC, *).

Readable column-major matrix of right Taylor derivatives, with at least `LDC * LXI` double-precision elements.

## `XI`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

Readable breakpoint vector with at least `LXI + 1` elements; it defines the piecewise-polynomial intervals.

## `LXI`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input number of polynomial pieces; it must be positive.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input B-spline order; it must be positive.

## `ID`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input derivative order, constrained to `0..K-1`; zero selects the spline itself.

## `X1`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input first integration endpoint. The routine reverses the sign when `X1 > X2`.

## `X2`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input second integration endpoint.

## `TOL`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Input requested quadrature tolerance. The selected source requires it between the double-precision machine floor and `0.1`.

## `QUAD`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

Writable output integral of `F(X)` times the selected B-spline derivative.

## `IERR`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Writable status: `1` means normal completion and `2` means some subinterval did not meet `TOL`. Invalid arguments enter the legacy error path.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Workspace and array requirements

- `LDC`: not a workspace argument
- `C`: not a workspace argument
- `XI`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::quadrature::dpfqad`
- Original SLATEC routine: `DPFQAD`
- Native symbol: `dpfqad_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DPFQAD](https://www.netlib.org/slatec/src/dpfqad.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
