# Purpose

Subroutine POIS3D solves the linear system of equations

# Description

This canonical unsafe binding exposes original SLATEC routine `POIS3D`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [POIS3D](https://www.netlib.org/slatec/fishfft/pois3d.f).

# Arguments

## `LPEROD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the values that X(0,J,K) and X(L+1,J,K) are assumed to have. = 0 If X(0,J,K) = X(L,J,K) and X(L+1,J,K) = X(1,J,K). = 1 If X(0,J,K) = X(L+1,J,K) = 0. = 2 If X(0,J,K) = 0 and X(L+1,J,K) = X(L-1,J,K). = 3 If X(0,J,K) = X(2,J,K) and X(L+1,J,K) = X(L-1,J,K). = 4 If X(0,J,K) = X(2,J,K) and X(L+1,J,K) = 0.

## `L`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of unknowns in the I-direction. L must be at least 3. M=N) LPEROD MPEROD T(MSECS) E 16 0 0 272 1. E-13 15 1 1 287 4. E-13 17 3 3 338 2. E-13 32 0 0 1755 2.

## `C1`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

+ C2*(X(I,J-1,K)-2. *X(I,J,K)+X(I,J+1,K)) + A(K)*X(I,J,K-1)+B(K)*X(I,J,K)+C(K)*X(I,J,K+1) = F(I,J,K) for I=1,2,. ,L , J=1,2,. ,M , and K=1,2,. ,N. The indices K-1 and K+1 are evaluated modulo N, i.

## `MPEROD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the values that X(I,0,K) and X(I,M+1,K) are assumed to have. = 0 If X(I,0,K) = X(I,M,K) and X(I,M+1,K) = X(I,1,K). = 1 If X(I,0,K) = X(I,M+1,K) = 0. = 2 If X(I,0,K) = 0 and X(I,M+1,K) = X(I,M-1,K). = 3 If X(I,0,K) = X(I,2,K) and X(I,M+1,K) = X(I,M-1,K). = 4 If X(I,0,K) = X(I,2,K) and X(I,M+1,K) = 0.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of unknowns in the J-direction. M must be at least 3.

## `C2`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

The real constant which appears in the above equation.

## `NPEROD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

0 If A(1) and C(N) are not zero. = 1 If A(1) = C(N) = 0.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of unknowns in the K-direction. N must be at least 3.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

One-dimensional arrays of length N that specify the coefficients in the linear equations given above. If NPEROD = 0 the array elements must not depend upon the index K, but must be constant. Specifically, the subroutine checks the following condition C(1) C(K) = -0. 5*B(K) = 1, K=1,2,. ,N and, when NPEROD = 1 C(N) = 0 C(1) = 2. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

One-dimensional arrays of length N that specify the coefficients in the linear equations given above. If NPEROD = 0 the array elements must not depend upon the index K, but must be constant. Specifically, the subroutine checks the following condition B(1) for K=1,2,. ,N.

## `C`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

One-dimensional arrays of length N that specify the coefficients in the linear equations given above. If NPEROD = 0 the array elements must not depend upon the index K, but must be constant. Specifically, the subroutine checks the following condition C(1).

## `LDIMF`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The row (or first) dimension of the three-dimensional array F as it appears in the program calling POIS3D. This parameter is used to specify the variable dimension of F. LDIMF must be at least L.

## `MDIMF`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The column (or second) dimension of the three-dimensional array F as it appears in the program calling POIS3D. This parameter is used to specify the variable dimension of F. MDIMF must be at least M.

## `F`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 3; dimensions (LDIMF, MDIMF, *).

A three-dimensional array that specifies the values of the right side of the linear system of equations given above. F must be dimensioned at least L x M x N. Contains the solution X.

## `IERROR`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An error flag that indicates invalid input parameters. Except for number zero, a solution is not attempted. = 0 No error = 1 If LPEROD. LT. 0 or. GT.

## `W`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array that must be provided by the user for work space. The length of W must be at least 30 + L + M + 2*N + MAX(L,M,N) + 7*(INT((L+1)/2) + INT((M+1)/2)).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error |
| `IERROR` | `1` | 1 If LPEROD .LT. 0 or .GT. 4 |
| `IERROR` | `2` | 2 If L .LT. 3 |
| `IERROR` | `3` | 3 If MPEROD .LT. 0 or .GT. 4 |
| `IERROR` | `4` | 4 If M .LT. 3 |
| `IERROR` | `5` | 5 If NPEROD .LT. 0 or .GT. 1 |
| `IERROR` | `6` | 6 If N .LT. 3 |
| `IERROR` | `7` | 7 If LDIMF .LT. L |
| `IERROR` | `8` | 8 If MDIMF .LT. M |
| `IERROR` | `9` | 9 If A(K) .NE. C(1) or C(K) .NE. C(1) or B(I) .NE.B(1) |
| `IERROR` | `1` | ,2,...,N. |
| `IERROR` | `10` | 10 If NPEROD = 1 and A(1) .NE. 0 or C(N) .NE. 0 Since this is the only means of indicating a possibly incorrect call to POIS3D, the user should test IERROR after the call. |

# Workspace and array requirements

- `A`: not a workspace argument
- `B`: not a workspace argument
- `C`: not a workspace argument
- `LDIMF`: not a workspace argument
- `F`: not a workspace argument
- `W`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::pois3d`
- Original SLATEC routine: `POIS3D`
- Native symbol: `pois3d_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [POIS3D](https://www.netlib.org/slatec/fishfft/pois3d.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
