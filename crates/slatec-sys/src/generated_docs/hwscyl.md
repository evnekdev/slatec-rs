# Purpose

Subroutine HWSCYL solves a finite difference approximation to the Helmholtz equation in cylindrical coordinates: (1/R)(d/dR)(R(dU/dR)) + (d/dZ)(dU/dZ) + (LAMBDA/R**2)U = F(R,Z) This modified Helmholtz equation results from the Fourier transform of the three-dimensional Poisson equation. * * * * * * * Parameter Description * * * * * * * * * *

# Description

This canonical unsafe binding exposes original SLATEC routine `HWSCYL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HWSCYL](https://www.netlib.org/slatec/fishfft/hwscyl.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of R, i. e. , A. LE. R. B.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of R, i. e. , A. LE. R. B.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of panels into which the interval (A,B) is subdivided. Hence, there will be M+1 grid points in the R-direction given by R(I) = A+(I-1)DR, for I = 1,2,. ,M+1, where DR = (B-A)/M is the panel width. M must be greater than 3.

## `MBDCND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the type of boundary conditions at R = A and R = B. = 1 If the solution is specified at R = A and R = B. = 2 If the solution is specified at R = A and the derivative of the solution with respect to R is specified at R = B. = 3 If the derivative of the solution with respect to R is specified at R = A (see note below) and R = B. = 4 If the derivative of the solution with respect to R is specified at R = A (see note below) and the solution is = 5 If the solution is unspecified at R = A = 0 and the solution is specified at R = B. = 6 If the solution is unspecified at R = A = 0 and the NOTE: If A = 0, do not use MBDCND = 3 or 4, but instead use 1,2,5, or 6.

## `BDA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = A. When MBDCND = 3 or 4, (d/dR)U(A,Z(J)), J = 1,2,. ,N+1. When MBDCND has any other value, BDA is a dummy variable.

## `BDB`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to R at R = B. When MBDCND = 2,3, or 6, (d/dR)U(B,Z(J)), J = 1,2,. ,N+1. When MBDCND has any other value, BDB is a dummy variable.

## `C`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of Z, i. e. , C. LE. Z. D.

## `D`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of Z, i. e. , C. LE. Z. D.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of panels into which the interval (C,D) is subdivided. Hence, there will be N+1 grid points in the Z-direction given by Z(J) = C+(J-1)DZ, for J = 1,2,. ,N+1, where DZ = (D-C)/N is the panel width. N must be greater than 3.

## `NBDCND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the type of boundary conditions at Z = C and Z = D. = 0 If the solution is periodic in Z, i. e. , U(I,1) = U(I,N+1). = 1 If the solution is specified at Z = C and Z = D. = 2 If the solution is specified at Z = C and the derivative of the solution with respect to Z is specified at Z = D.

## `BDC`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Z at Z = C. When NBDCND = 3 or 4, (d/dZ)U(R(I),C), I = 1,2,. ,M+1. When NBDCND has any other value, BDC is a dummy variable.

## `BDD`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to Z at Z = D. When NBDCND = 2 or 3, (d/dZ)U(R(I),D), I = 1,2,. ,M+1. When NBDCND has any other value, BDD is a dummy variable.

## `ELMBDA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The constant LAMBDA in the Helmholtz equation. If LAMBDA. GT. 0, a solution may not exist. However, HWSCYL will attempt to find a solution. LAMBDA must be zero when.

## `F`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (IDIMF, *).

A two-dimensional array that specifies the values of the right side of the Helmholtz equation and boundary data (if any). For I = 2,3,. ,M and J = 2,3,. ,N F(R(I),Z(J)). On the boundaries F is defined by must be dimensioned at least (M+1)*(N+1). NOTE If the table calls for both the solution U and the right side F at a corner then the solution must be specified.

## `IDIMF`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The row (or first) dimension of the array F as it appears in the program calling HWSCYL. This parameter is used to specify the variable dimension of F. IDIMF must be at least M+1.

## `PERTRB`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

If one specifies a combination of periodic, derivative, and unspecified boundary conditions for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a constant, calculated and subtracted from F, which ensures that a solution exists. HWSCYL then computes this solution, which is a least squares solution to the original approximation. This solution plus any constant is also a solution. Hence, the solution is not unique. The value of PERTRB should be small compared to the right side F.

## `IERROR`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An error flag which indicates invalid input parameters. Except for numbers 0 and 11, a solution is not attempted. = 0 No error. = 1 A. LT. 0.

## `W`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array that must be provided by the user for work space. W may require up to 4*(N+1) + (13 + INT(log2(N+1)))*(M+1) locations. The actual number of locations used is computed by HWSCYL and is returned in location contains the required length of W.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error. |
| `IERROR` | `1` | 1 A .LT. 0 . |
| `IERROR` | `2` | 2 A .GE. B. |
| `IERROR` | `3` | 3 MBDCND .LT. 1 or MBDCND .GT. 6 . |
| `IERROR` | `4` | 4 C .GE. D. |
| `IERROR` | `5` | 5 N .LE. 3 |
| `IERROR` | `6` | 6 NBDCND .LT. 0 or NBDCND .GT. 4 . |
| `IERROR` | `7` | 7 A = 0, MBDCND = 3 or 4 . |
| `IERROR` | `8` | 8 A .GT. 0, MBDCND .GE. 5 . |
| `IERROR` | `9` | 9 A = 0, LAMBDA .NE. 0, MBDCND .GE. 5 . |
| `IERROR` | `10` | 10 IDIMF .LT. M+1 . |
| `IERROR` | `11` | 11 LAMBDA .GT. 0 . |
| `IERROR` | `12` | 12 M .LE. 3 Since this is the only means of indicating a possibly incorrect call to HWSCYL, the user should test IERROR after the call. |

# Workspace and array requirements

- `BDA`: not a workspace argument
- `BDB`: not a workspace argument
- `BDC`: not a workspace argument
- `BDD`: not a workspace argument
- `F`: not a workspace argument
- `IDIMF`: not a workspace argument
- `W`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hwscyl`
- Original SLATEC routine: `HWSCYL`
- Native symbol: `hwscyl_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [HWSCYL](https://www.netlib.org/slatec/fishfft/hwscyl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
