# Purpose

HSTSSP solves the standard five-point finite difference approximation on a staggered grid to the Helmholtz equation in spherical coordinates and on the surface of the unit sphere (radius of 1) (1/SIN(THETA))(d/dTHETA)(SIN(THETA)(dU/dTHETA)) + (1/SIN(THETA)**2)(d/dPHI)(dU/dPHI) + LAMBDA*U = F(THETA,PHI) where THETA is colatitude and PHI is longitude. * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * Parameter Description * * * * * * * * * *

# Description

This canonical unsafe binding exposes original SLATEC routine `HSTSSP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HSTSSP](https://www.netlib.org/slatec/fishfft/hstssp.f).

# Arguments

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of THETA (colatitude), i. e. A. LE. THETA. B.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of THETA (colatitude), i. e. A. LE. THETA. B.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of grid points in the interval (A,B). The grid points in the THETA-direction are given by THETA(I) = A + (I-0. 5)DTHETA for I=1,2,. ,M where DTHETA =(B-A)/M. M must be greater than 2.

## `MBDCND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the type of boundary conditions at THETA = A and THETA = B. = 1 If the solution is specified at THETA = A and THETA = B. (see note 3 below) = 2 If the solution is specified at THETA = A and the derivative of the solution with respect to THETA is specified at THETA = B (see notes 2 and 3 below). = 3 If the derivative of the solution with respect to THETA is specified at THETA = A (see notes 1, 2 below) and THETA = B. = 4 If the derivative of the solution with respect to THETA is specified at THETA = A (see notes 1 and 2 below) and the solution is specified at THETA = B. = 5 If the solution is unspecified at THETA = A = 0 and the solution is specified at THETA = B.

## `BDA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length N that specifies the boundary values (if any) of the solution at THETA = A. When U(A,PHI(J)) , J=1,2,. ,N. When MBDCND = 3, 4, or 8, (d/dTHETA)U(A,PHI(J)) , J=1,2,. When MBDCND has any other value, BDA is a dummy variable.

## `BDB`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length N that specifies the boundary values of the solution at THETA = B. When MBDCND = 1,4, or 5, U(B,PHI(J)) , J=1,2,. ,N. When MBDCND = 2,3, or 6, (d/dTHETA)U(B,PHI(J)) , J=1,2,. When MBDCND has any other value, BDB is a dummy variable.

## `C`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of PHI (longitude), i. e. C. LE. PHI. D.

## `D`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of PHI (longitude), i. e. C. LE. PHI. D.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of unknowns in the interval (C,D). The unknowns in the PHI-direction are given by PHI(J) = C + (J-0. 5)DPHI, J=1,2,. ,N, where DPHI = (D-C)/N. N must be greater than 2.

## `NBDCND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the type of boundary conditions at PHI = C and PHI = D. = 0 If the solution is periodic in PHI, i. e. U(I,J) = U(I,N+J). = 1 If the solution is specified at PHI = C and PHI = D (see note below). = 2 If the solution is specified at PHI = C and the derivative of the solution with respect to PHI is specified at = 3 If the derivative of the solution with respect to PHI is specified at PHI = C and PHI = D.

## `BDC`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one dimensional array of length M that specifies the boundary values of the solution at PHI = C. When NBDCND = 1 or 2, U(THETA(I),C) , I=1,2,. ,M. When NBDCND = 3 or 4, (d/dPHI)U(THETA(I),C), I=1,2,. When NBDCND = 0, BDC is a dummy variable.

## `BDD`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length M that specifies the boundary values of the solution at PHI = D. When NBDCND = 1 or 4, U(THETA(I),D) , I=1,2,. ,M. When NBDCND = 2 or 3, (d/dPHI)U(THETA(I),D) , I=1,2,. When NBDCND = 0, BDD is a dummy variable.

## `ELMBDA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The constant LAMBDA in the Helmholtz equation. If LAMBDA is greater than 0, a solution may not exist. However, HSTSSP will attempt to find a solution.

## `F`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (IDIMF, *).

A two-dimensional array that specifies the values of the right side of the Helmholtz equation. For I=1,2,. ,M and J=1,2,. ,N F(THETA(I),PHI(J)). must be dimensioned at least M X N. Contains the solution U(I,J) of the finite difference approximation for the grid point (THETA(I),PHI(J)) for I=1,2,.

## `IDIMF`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The row (or first) dimension of the array F as it appears in the program calling HSTSSP. This parameter is used to specify the variable dimension of F. IDIMF must be at least M.

## `PERTRB`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

If a combination of periodic, derivative, or unspecified boundary conditions is specified for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a con- stant, calculated and subtracted from F, which ensures that a solution exists. HSTSSP then computes this solution, which is a least squares solution to the original approximation. This solution plus any constant is also a solution; hence, the solution is not unique. The value of should be small compared to the right side F. Otherwise, a solution is obtained to an essentially different problem.

## `IERROR`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An error flag that indicates invalid input parameters. Except for numbers 0 and 14, a solution is not attempted. = 0 No error = 1 A. LT. 0 or B. GT.

## `W`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array that must be provided by the user for work space. W may require up to 13M + 4N + M*INT(log2(N)) locations. The actual number of locations used is computed by HSTSSP and is returned in the location W(1). contains the required length of W.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error |
| `IERROR` | `1` | 1 A .LT. 0 or B .GT. PI |
| `IERROR` | `2` | 2 A .GE. B |
| `IERROR` | `3` | 3 MBDCND .LT. 1 or MBDCND .GT. 9 |
| `IERROR` | `4` | 4 C .GE. D |
| `IERROR` | `5` | 5 N .LE. 2 |
| `IERROR` | `6` | 6 NBDCND .LT. 0 or NBDCND .GT. 4 |
| `IERROR` | `7` | 7 A .GT. 0 and MBDCND = 5, 6, or 9 |
| `IERROR` | `8` | 8 A = 0 and MBDCND = 3, 4, or 8 |
| `IERROR` | `9` | 9 B .LT. PI and MBDCND .GE. 7 |
| `IERROR` | `10` | 10 B = PI and MBDCND = 2,3, or 6 |
| `IERROR` | `11` | 11 MBDCND .GE. 5 and NDBCND = 1, 2, or 4 |
| `IERROR` | `12` | 12 IDIMF .LT. M |
| `IERROR` | `13` | 13 M .LE. 2 |
| `IERROR` | `14` | 14 LAMBDA .GT. 0 Since this is the only means of indicating a possibly incorrect call to HSTSSP, the user should test IERROR after the call. |

# Workspace and array requirements

- `BDA`: not a workspace argument
- `BDB`: not a workspace argument
- `BDC`: not a workspace argument
- `BDD`: not a workspace argument
- `F`: not a workspace argument
- `IDIMF`: not a workspace argument
- `W`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hstssp`
- Original SLATEC routine: `HSTSSP`
- Native symbol: `hstssp_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [HSTSSP](https://www.netlib.org/slatec/fishfft/hstssp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
