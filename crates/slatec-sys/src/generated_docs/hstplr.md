# Purpose

HSTPLR solves the standard five-point finite difference approximation on a staggered grid to the Helmholtz equation in polar coordinates (1/R)(d/DR)(R(dU/DR)) + (1/R**2)(d/dTHETA)(dU/dTHETA) + LAMBDA*U = F(R,THETA) * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * Parameter Description * * * * * * * * * *

# Description

This canonical unsafe binding exposes original SLATEC routine `HSTPLR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HSTPLR](https://www.netlib.org/slatec/fishfft/hstplr.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of R, i. e. A. LE. R. B.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of R, i. e. A. LE. R. B.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of grid points in the interval (A,B). The grid points in the R-direction are given by R(I) = A + (I-0. 5)DR for I=1,2,. ,M where DR =(B-A)/M. M must be greater than 2.

## `MBDCND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the type of boundary conditions at R = A and R = B. = 1 If the solution is specified at R = A and R = B. = 2 If the solution is specified at R = A and the derivative of the solution with respect to R is specified at R = B. (see note 1 below) = 3 If the derivative of the solution with respect to R is specified at R = A (see note 2 below) and R = B. = 4 If the derivative of the solution with respect to R is specified at R = A (see note 2 below) and the solution is = 5 If the solution is unspecified at R = A = 0 and the solution = 6 If the solution is unspecified at R = A = 0 and the NOTE 1: If A = 0, MBDCND = 2, and NBDCND = 0 or 3, the system of equations to be solved is singular. The unique solution is determined by extrapolation to the specification of U(0,THETA(1)).

## `BDA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length N that specifies the boundary values (if any) of the solution at R = A. When MBDCND = 1 or 2, U(A,THETA(J)) , J=1,2,. ,N. When MBDCND = 3 or 4, (d/dR)U(A,THETA(J)) , J=1,2,. When MBDCND = 5 or 6, BDA is a dummy variable.

## `BDB`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length N that specifies the boundary values of the solution at R = B. When MBDCND = 1,4, or 5, U(B,THETA(J)) , J=1,2,. ,N. When MBDCND = 2,3, or 6, (d/dR)U(B,THETA(J)) , J=1,2,.

## `C`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of THETA, i. e. C. LE. THETA. D.

## `D`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of THETA, i. e. C. LE. THETA. D.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of unknowns in the interval (C,D). The unknowns in the THETA-direction are given by THETA(J) = C + (J-0. 5)DT, J=1,2,. ,N, where DT = (D-C)/N. N must be greater than 2.

## `NBDCND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the type of boundary conditions at THETA = C and THETA = D. = 0 If the solution is periodic in THETA, i. e. U(I,J) = U(I,N+J). = 1 If the solution is specified at THETA = C and THETA = D (see note below). = 2 If the solution is specified at THETA = C and the derivative of the solution with respect to THETA is specified at = 3 If the derivative of the solution with respect to THETA is specified at THETA = C and THETA = D.

## `BDC`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one dimensional array of length M that specifies the boundary values of the solution at THETA = C. When NBDCND = 1 or 2, U(R(I),C) , I=1,2,. ,M. When NBDCND = 3 or 4, (d/dTHETA)U(R(I),C), I=1,2,. When NBDCND = 0, BDC is a dummy variable.

## `BDD`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length M that specifies the boundary values of the solution at THETA = D. When NBDCND = 1 or 4, U(R(I),D) , I=1,2,. ,M. When NBDCND = 2 or 3, (d/dTHETA)U(R(I),D) , I=1,2,. When NBDCND = 0, BDD is a dummy variable.

## `ELMBDA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The constant LAMBDA in the Helmholtz equation. If LAMBDA is greater than 0, a solution may not exist. However, HSTPLR will attempt to find a solution.

## `F`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (IDIMF, *).

A two-dimensional array that specifies the values of the right side of the Helmholtz equation. For I=1,2,. ,M and J=1,2,. ,N F(R(I),THETA(J)). must be dimensioned at least M X N. Contains the solution U(I,J) of the finite difference approximation for the grid point (R(I),THETA(J)) for I=1,2,.

## `IDIMF`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The row (or first) dimension of the array F as it appears in the program calling HSTPLR. This parameter is used to specify the variable dimension of F. IDIMF must be at least M.

## `PERTRB`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a con- stant, calculated and subtracted from F, which ensures that a solution exists. HSTPLR then computes this solution, which is a least squares solution to the original approximation. This solution plus any constant is also a solution; hence, the solution is not unique. The value of should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem.

## `IERROR`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An error flag that indicates invalid input parameters. Except for numbers 0 and 11, a solution is not attempted. = 0 No error = 1 A. LT. 0 = 2 A. GE.

## `W`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array that must be provided by the user for work space. W may require up to 13M + 4N + M*INT(log2(N)) locations. The actual number of locations used is computed by HSTPLR and is returned in the location W(1). contains the required length of W.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error |
| `IERROR` | `1` | 1 A .LT. 0 |
| `IERROR` | `2` | 2 A .GE. B |
| `IERROR` | `3` | 3 MBDCND .LT. 1 or MBDCND .GT. 6 |
| `IERROR` | `4` | 4 C .GE. D |
| `IERROR` | `5` | 5 N .LE. 2 |
| `IERROR` | `6` | 6 NBDCND .LT. 0 or NBDCND .GT. 4 |
| `IERROR` | `7` | 7 A = 0 and MBDCND = 3 or 4 |
| `IERROR` | `8` | 8 A .GT. 0 and MBDCND .GE. 5 |
| `IERROR` | `9` | 9 MBDCND .GE. 5 and NBDCND .NE. 0 or 3 |
| `IERROR` | `10` | 10 IDIMF .LT. M |
| `IERROR` | `11` | 11 LAMBDA .GT. 0 |
| `IERROR` | `12` | 12 M .LE. 2 Since this is the only means of indicating a possibly incorrect call to HSTPLR, the user should test IERROR after the call. |

# Workspace and array requirements

- `BDA`: not a workspace argument
- `BDB`: not a workspace argument
- `BDC`: not a workspace argument
- `BDD`: not a workspace argument
- `F`: not a workspace argument
- `IDIMF`: not a workspace argument
- `W`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hstplr`
- Original SLATEC routine: `HSTPLR`
- Native symbol: `hstplr_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [HSTPLR](https://www.netlib.org/slatec/fishfft/hstplr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
