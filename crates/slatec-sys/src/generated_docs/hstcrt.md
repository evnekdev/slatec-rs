# Purpose

HSTCRT solves the standard five-point finite difference approximation on a staggered grid to the Helmholtz equation in Cartesian coordinates (d/dX)(dU/dX) + (d/dY)(dU/dY) + LAMBDA*U = F(X,Y) * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * Parameter Description * * * * * * * * * *

# Description

This canonical unsafe binding exposes original SLATEC routine `HSTCRT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HSTCRT](https://www.netlib.org/slatec/fishfft/hstcrt.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of X, i. e. A. LE. X. B.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of X, i. e. A. LE. X. B.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of grid points in the interval (A,B). The grid points in the X-direction are given by X(I) = A + (I-0. 5)dX for I=1,2,. ,M where dX =(B-A)/M. M must be greater than 2.

## `MBDCND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the type of boundary conditions at X = A and X = B. = 0 If the solution is periodic in X, U(M+I,J) = U(I,J). = 1 If the solution is specified at X = A and X = B. = 2 If the solution is specified at X = A and the derivative of the solution with respect to X is specified at X = B. = 3 If the derivative of the solution with respect to X is = 4 If the derivative of the solution with respect to X is specified at X = A and the solution is specified at X = B.

## `BDA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length N that specifies the boundary values (if any) of the solution at X = A. When MBDCND = 1 or 2, U(A,Y(J)) , J=1,2,. ,N. When MBDCND = 3 or 4, (d/dX)U(A,Y(J)) , J=1,2,.

## `BDB`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length N that specifies the boundary values of the solution at X = B. When MBDCND = 1 or 4 U(B,Y(J)) , J=1,2,. ,N. When MBDCND = 2 or 3 (d/dX)U(B,Y(J)) , J=1,2,.

## `C`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of Y, i. e. C. LE. Y. D.

## `D`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of Y, i. e. C. LE. Y. D.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of unknowns in the interval (C,D). The unknowns in the Y-direction are given by Y(J) = C + (J-0. 5)DY, J=1,2,. ,N, where DY = (D-C)/N. N must be greater than 2.

## `NBDCND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the type of boundary conditions at Y = C and Y = D. = 0 If the solution is periodic in Y, i. e. U(I,J) = U(I,N+J). = 1 If the solution is specified at Y = C and Y = D. = 2 If the solution is specified at Y = C and the derivative of the solution with respect to Y is specified at Y = D.

## `BDC`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one dimensional array of length M that specifies the boundary values of the solution at Y = C. When NBDCND = 1 or 2, U(X(I),C) , I=1,2,. ,M. When NBDCND = 3 or 4, (d/dY)U(X(I),C), I=1,2,. When NBDCND = 0, BDC is a dummy variable.

## `BDD`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length M that specifies the boundary values of the solution at Y = D. When NBDCND = 1 or 4, U(X(I),D) , I=1,2,. ,M. When NBDCND = 2 or 3, (d/dY)U(X(I),D) , I=1,2,. When NBDCND = 0, BDD is a dummy variable.

## `ELMBDA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The constant LAMBDA in the Helmholtz equation. If LAMBDA is greater than 0, a solution may not exist. However, HSTCRT will attempt to find a solution.

## `F`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (IDIMF, *).

A two-dimensional array that specifies the values of the right side of the Helmholtz equation. For I=1,2,. ,M and J=1,2,. ,N F(X(I),Y(J)). must be dimensioned at least M X N. Contains the solution U(I,J) of the finite difference approximation for the grid point (X(I),Y(J)) for I=1,2,.

## `IDIMF`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The row (or first) dimension of the array F as it appears in the program calling HSTCRT. This parameter is used to specify the variable dimension of F. IDIMF must be at least M.

## `PERTRB`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

If a combination of periodic or derivative boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a constant, calculated and subtracted from F, which ensures that a solution exists. HSTCRT then computes this solution, which is a least squares solution to the original approximation. This solution plus any constant is also a solution; hence, the solution is not unique. The value of PERTRB should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem.

## `IERROR`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An error flag that indicates invalid input parameters. Except for numbers 0 and 6, a solution is not attempted. = 0 No error = 1 A. GE. B = 2 MBDCND. LT.

## `W`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array that must be provided by the user for work space. W may require up to 13M + 4N + M*INT(log2(N)) locations. The actual number of locations used is computed by HSTCRT and is returned in the location W(1). contains the required length of W.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error |
| `IERROR` | `1` | 1 A .GE. B |
| `IERROR` | `2` | 2 MBDCND .LT. 0 or MBDCND .GT. 4 |
| `IERROR` | `3` | 3 C .GE. D |
| `IERROR` | `4` | 4 N .LE. 2 |
| `IERROR` | `5` | 5 NBDCND .LT. 0 or NBDCND .GT. 4 |
| `IERROR` | `6` | 6 LAMBDA .GT. 0 |
| `IERROR` | `7` | 7 IDIMF .LT. M |
| `IERROR` | `8` | 8 M .LE. 2 Since this is the only means of indicating a possibly incorrect call to HSTCRT, the user should test IERROR after the call. |

# Workspace and array requirements

- `BDA`: not a workspace argument
- `BDB`: not a workspace argument
- `BDC`: not a workspace argument
- `BDD`: not a workspace argument
- `F`: not a workspace argument
- `IDIMF`: not a workspace argument
- `W`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hstcrt`
- Original SLATEC routine: `HSTCRT`
- Native symbol: `hstcrt_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [HSTCRT](https://www.netlib.org/slatec/fishfft/hstcrt.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
