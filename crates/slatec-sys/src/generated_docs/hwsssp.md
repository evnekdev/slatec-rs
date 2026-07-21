# Purpose

Subroutine HWSSSP solves a finite difference approximation to the Helmholtz equation in spherical coordinates and on the surface of the unit sphere (radius of 1): (1/SIN(THETA))(d/dTHETA)(SIN(THETA)(dU/dTHETA)) + (1/SIN(THETA)**2)(d/dPHI)(dU/dPHI) + LAMBDA*U = F(THETA,PHI) Where THETA is colatitude and PHI is longitude. * * * * * * * Parameter Description * * * * * * * * * *

# Description

This canonical unsafe binding exposes original SLATEC routine `HWSSSP`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [HWSSSP](https://www.netlib.org/slatec/fishfft/hwsssp.f).

# Arguments

## `TS`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of THETA (colatitude), i. e. , TS. LE. THETA. TF.

## `TF`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of THETA (colatitude), i. e. , TS. LE. THETA. TF.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of panels into which the interval (TS,TF) is subdivided. Hence, there will be M+1 grid points in the THETA-direction given by THETA(I) = (I-1)DTHETA+TS for I = 1,2,. ,M+1, where DTHETA = (TF-TS)/M is the panel width. must be greater than 5.

## `MBDCND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the type of boundary condition at THETA = TS and THETA = TF. = 1 If the solution is specified at THETA = TS and THETA = TF. = 2 If the solution is specified at THETA = TS and the derivative of the solution with respect to THETA is specified at THETA = TF (see note 2 below). = 3 If the derivative of the solution with respect to THETA is specified at THETA = TS and THETA = TF (see notes 1,2 = 4 If the derivative of the solution with respect to THETA is specified at THETA = TS (see note 1 below) and the solution is specified at THETA = TF. = 5 If the solution is unspecified at THETA = TS = 0 and the = 6 If the solution is unspecified at THETA = TS = 0 and the = 7 If the solution is specified at THETA = TS and the solution is unspecified at THETA = TF = PI. = 8 If the derivative of the solution with respect to THETA is = 9 If the solution is unspecified at THETA = TS = 0 and NOTES: 1.

## `BDTS`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TS. When MBDCND = 3,4, or 8, (d/dTHETA)U(TS,PHI(J)), J = 1,2,. ,N+1. When MBDCND has any other value, BDTS is a dummy variable.

## `BDTF`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length N+1 that specifies the values of the derivative of the solution with respect to THETA at THETA = TF. When MBDCND = 2,3, or 6, (d/dTHETA)U(TF,PHI(J)), J = 1,2,. ,N+1. When MBDCND has any other value, BDTF is a dummy variable.

## `PS`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of PHI (longitude), i. e. , PS. LE. PHI. PF.

## `PF`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The range of PHI (longitude), i. e. , PS. LE. PHI. PF.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of panels into which the interval (PS,PF) is subdivided. Hence, there will be N+1 grid points in the PHI-direction given by PHI(J) = (J-1)DPHI+PS for J = 1,2,. ,N+1, where DPHI = (PF-PS)/N is the panel width. must be greater than 4.

## `NBDCND`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the type of boundary condition at PHI = PS and PHI = PF. = 0 If the solution is periodic in PHI, i. e. , U(I,J) = U(I,N+J). = 1 If the solution is specified at PHI = PS and PHI = PF (see note below). = 2 If the solution is specified at PHI = PS (see note below) and the derivative of the solution with respect to PHI is specified at PHI = PF.

## `BDPS`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to PHI at PHI = PS. When NBDCND = 3 or 4, (d/dPHI)U(THETA(I),PS), I = 1,2,. ,M+1. When NBDCND has any other value, BDPS is a dummy variable.

## `BDPF`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array of length M+1 that specifies the values of the derivative of the solution with respect to PHI at PHI = PF. When NBDCND = 2 or 3, (d/dPHI)U(THETA(I),PF), I = 1,2,. ,M+1. When NBDCND has any other value, BDPF is a dummy variable.

## `ELMBDA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The constant LAMBDA in the Helmholtz equation. If LAMBDA. GT. 0, a solution may not exist. However, HWSSSP will attempt to find a solution.

## `F`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (IDIMF, *).

A two-dimensional array that specifies the value of the right side of the Helmholtz equation and boundary values (if any). For I = 2,3,. ,M and J = 2,3,. ,N F(THETA(I),PHI(J)). On the boundaries F is defined by must be dimensioned at least (M+1)*(N+1). NOTE* If the table calls for both the solution U and the right side F at a corner then the solution must be specified.

## `IDIMF`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The row (or first) dimension of the array F as it appears in the program calling HWSSSP. This parameter is used to specify the variable dimension of F. IDIMF must be at least M+1.

## `PERTRB`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

If one specifies a combination of periodic, derivative or unspecified boundary conditions for a Poisson equation (LAMBDA = 0), a solution may not exist. PERTRB is a constant, calculated and subtracted from F, which ensures that a solution exists. HWSSSP then computes this solution, which is a least squares solution to the original approximation. This solution is not unique and is unnormalized. The value of PERTRB should be small compared to the right side F. Otherwise , a solution is obtained to an essentially different problem.

## `IERROR`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An error flag that indicates invalid input parameters. Except for numbers 0 and 8, a solution is not attempted. = 0 No error = 1 TS. LT. 0 or TF. GT.

## `W`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array that must be provided by the user for work space. W may require up to 4*(N+1)+(16+INT(log2(N+1)))(M+1) locations. The actual number of locations used is computed by HWSSSP and is output in location W(1). INT( ) denotes the FORTRAN integer function. Contains intermediate values that must not be destroyed if HWSSSP will be called again with INTL = 1. W(1) contains the required length of W.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error |
| `IERROR` | `1` | 1 TS.LT.0 or TF.GT.PI |
| `IERROR` | `2` | 2 TS.GE.TF |
| `IERROR` | `3` | 3 MBDCND.LT.1 or MBDCND.GT.9 |
| `IERROR` | `4` | 4 PS.LT.0 or PS.GT.PI+PI |
| `IERROR` | `5` | 5 PS.GE.PF |
| `IERROR` | `6` | 6 N.LT.5 |
| `IERROR` | `7` | 7 M.LT.5 |
| `IERROR` | `8` | 8 NBDCND.LT.0 or NBDCND.GT.4 |
| `IERROR` | `9` | 9 ELMBDA.GT.0 |
| `IERROR` | `10` | 10 IDIMF.LT.M+1 |
| `IERROR` | `11` | 11 NBDCND equals 1,2 or 4 and MBDCND.GE.5 |
| `IERROR` | `12` | 12 TS.EQ.0 and MBDCND equals 3,4 or 8 |
| `IERROR` | `13` | 13 TF.EQ.PI and MBDCND equals 2,3 or 6 |
| `IERROR` | `14` | 14 MBDCND equals 5,6 or 9 and TS.NE.0 |
| `IERROR` | `15` | 15 MBDCND.GE.7 and TF.NE.PI Since this is the only means of indicating a possibly incorrect call to HWSSSP, the user should test IERROR after a call. |

# Workspace and array requirements

- `BDTS`: not a workspace argument
- `BDTF`: not a workspace argument
- `BDPS`: not a workspace argument
- `BDPF`: not a workspace argument
- `F`: not a workspace argument
- `IDIMF`: not a workspace argument
- `W`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::hwsssp`
- Original SLATEC routine: `HWSSSP`
- Native symbol: `hwsssp_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank2,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [HWSSSP](https://www.netlib.org/slatec/fishfft/hwsssp.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
