# Purpose

Subroutine HWSCRT solves the standard five-point finite difference approximation to the Helmholtz equation in Cartesian coordinates: (d/dX)(dU/dX) + (d/dY)(dU/dY) + LAMBDA*U = F(X,Y). * * * * * * * Parameter Description * * * * * * * * * *

# Description

This canonical unsafe binding exposes original SLATEC routine `HWSCRT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HWSCRT](https://www.netlib.org/slatec/fishfft/hwscrt.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of X, i. e. , A. LE. X. B.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of X, i. e. , A. LE. X. B.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of panels into which the interval (A,B) is subdivided. Hence, there will be M+1 grid points in the X-direction given by X(I) = A+(I-1)DX for I = 1,2,. ,M+1, where DX = (B-A)/M is the panel width. M must be greater than 3.

## `MBDCND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the type of boundary conditions at X = A and X = B. = 0 If the solution is periodic in X, i. e. , U(I,J) = U(M+I,J). = 1 If the solution is specified at X = A and X = B. = 2 If the solution is specified at X = A and the derivative of the solution with respect to X is specified at X = B.

## `BDA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = A. When MBDCND = 3 or 4, (d/dX)U(A,Y(J)), J = 1,2,. ,N+1. When MBDCND has any other value, BDA is a dummy variable.

## `BDB`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to X at X = B. When MBDCND = 2 or 3, (d/dX)U(B,Y(J)), J = 1,2,. ,N+1. When MBDCND has any other value BDB is a dummy variable.

## `C`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of Y, i. e. , C. LE. Y. D.

## `D`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of Y, i. e. , C. LE. Y. D.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of panels into which the interval (C,D) is subdivided. Hence, there will be N+1 grid points in the Y-direction given by Y(J) = C+(J-1)DY for J = 1,2,. ,N+1, where DY = (D-C)/N is the panel width. N must be greater than 3.

## `NBDCND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the type of boundary conditions at Y = C and Y = D. = 0 If the solution is periodic in Y, i. e. , U(I,J) = U(I,N+J). = 1 If the solution is specified at Y = C and Y = D. = 2 If the solution is specified at Y = C and the derivative of the solution with respect to Y is specified at Y = D.

## `BDC`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Y at Y = C. When NBDCND = 3 or 4, (d/dY)U(X(I),C), I = 1,2,. ,M+1. When NBDCND has any other value, BDC is a dummy variable.

## `BDD`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Y at Y = D. When NBDCND = 2 or 3, (d/dY)U(X(I),D), I = 1,2,. ,M+1. When NBDCND has any other value, BDD is a dummy variable.

## `ELMBDA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The constant LAMBDA in the Helmholtz equation. If LAMBDA. GT. 0, a solution may not exist. However, HWSCRT will attempt to find a solution.

## `F`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (IDIMF, *).

A two-dimensional array which specifies the values of the right side of the Helmholtz equation and boundary values (if any). For I = 2,3,. ,M and J = 2,3,. ,N F(X(I),Y(J)). On the boundaries F is defined by must be dimensioned at least (M+1)*(N+1). NOTE: If the table calls for both the solution U and the right side F at a corner then the solution must be specified.

## `IDIMF`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The row (or first) dimension of the array F as it appears in the program calling HWSCRT. This parameter is used to specify the variable dimension of F. IDIMF must be at least M+1.

## `PERTRB`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

If a combination of periodic or derivative boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a constant, calculated and subtracted from F, which ensures that a solution exists. HWSCRT then computes this solution, which is a least squares solution to the original approximation. This solution plus any constant is also a solution. Hence, the solution is not unique. The value of should be small compared to the right side F.

## `IERROR`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An error flag that indicates invalid input parameters. Except for numbers 0 and 6, a solution is not attempted. = 0 No error. = 1 A. GE. B.

## `W`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array that must be provided by the user for work space. W may require up to 4*(N+1) + (13 + INT(log2(N+1)))*(M+1) locations. The actual number of locations used is computed by HWSCRT and is returned in location contains the required length of W.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error. |
| `IERROR` | `1` | 1 A .GE. B. |
| `IERROR` | `2` | 2 MBDCND .LT. 0 or MBDCND .GT. 4 . |
| `IERROR` | `3` | 3 C .GE. D. |
| `IERROR` | `4` | 4 N .LE. 3 |
| `IERROR` | `5` | 5 NBDCND .LT. 0 or NBDCND .GT. 4 . |
| `IERROR` | `6` | 6 LAMBDA .GT. 0 . |
| `IERROR` | `7` | 7 IDIMF .LT. M+1 . |
| `IERROR` | `8` | 8 M .LE. 3 Since this is the only means of indicating a possibly incorrect call to HWSCRT, the user should test IERROR after the call. |

# Workspace and array requirements

- `BDA`: not a workspace argument
- `BDB`: not a workspace argument
- `BDC`: not a workspace argument
- `BDD`: not a workspace argument
- `F`: not a workspace argument
- `IDIMF`: not a workspace argument
- `W`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hwscrt`
- Original SLATEC routine: `HWSCRT`
- Native symbol: `hwscrt_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [HWSCRT](https://www.netlib.org/slatec/fishfft/hwscrt.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
