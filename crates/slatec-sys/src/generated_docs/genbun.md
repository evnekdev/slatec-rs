# Purpose

Subroutine GENBUN solves the linear system of equations A(I)*X(I-1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) for I = 1,2,...,M and J = 1,2,...,N. The indices I+1 and I-1 are evaluated modulo M, i.e., X(0,J) = X(M,J) and X(M+1,J) = X(1,J), and X(I,0) may be equal to 0, X(I,2), or X(I,N) and X(I,N+1) may be equal to 0, X(I,N-1), or X(I,1) depending on an input parameter. * * * * * * * Parameter Description * * * * * * * * * *

# Description

This canonical unsafe binding exposes original SLATEC routine `GENBUN`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [GENBUN](https://www.netlib.org/slatec/fishfft/genbun.f).

# Arguments

## `NPEROD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the values that X(I,0) and X(I,N+1) are assumed to have. = 0 If X(I,0) = X(I,N) and X(I,N+1) = X(I,1). = 1 If X(I,0) = X(I,N+1) = 0. = 2 If X(I,0) = 0 and X(I,N+1) = X(I,N-1). = 3 If X(I,0) = X(I,2) and X(I,N+1) = X(I,N-1). = 4 If X(I,0) = X(I,2) and X(I,N+1) = 0.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of unknowns in the J-direction. N must be greater than 2.

## `MPEROD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

= 0 if A(1) and C(M) are not zero. = 1 if A(1) = C(M) = 0.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of unknowns in the I-direction. M must be greater than 2. N) MPEROD NPEROD T(MSECS) E 31 0 0 36 6. E-14 31 1 1 21 4. E-13 31 1 3 41 3. E-13 32 0 0 29 9.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

One-dimensional arrays of length M that specify the coefficients in the linear equations given above. If MPEROD = 0 the array elements must not depend upon the index I, but must be constant. Specifically, the subroutine checks the following condition C(1) C(I) = -0. 5*B(I) = 1, I=1,2,. ,M and, when MPEROD = 1 C(M) = 0 C(1) = 2. The solution X was substituted into the given sys- tem and, using double precision, a right side Y was computed.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

One-dimensional arrays of length M that specify the coefficients in the linear equations given above. If MPEROD = 0 the array elements must not depend upon the index I, but must be constant. Specifically, the subroutine checks the following condition B(1) for I=1,2,. ,M.

## `C`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

One-dimensional arrays of length M that specify the coefficients in the linear equations given above. If MPEROD = 0 the array elements must not depend upon the index I, but must be constant. Specifically, the subroutine checks the following condition C(1).

## `IDIMY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The row (or first) dimension of the two-dimensional array Y as it appears in the program calling GENBUN. This parameter is used to specify the variable dimension of Y. IDIMY must be at least M.

## `Y`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (IDIMY, *).

A two-dimensional array that specifies the values of the right side of the linear system of equations given above. Y must be dimensioned at least M*N. Contains the solution X.

## `IERROR`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An error flag that indicates invalid input parameters. Except for number zero, a solution is not attempted. = 0 No error. = 1 M. LE. 2 = 2 N.

## `W`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array that must be provided by the user for work space. W may require up to 4*N + (10 + INT(log2(N)))*M locations. The actual number of locations used is computed by GENBUN and is returned in location W(1). contains the required length of W.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error. |
| `IERROR` | `1` | 1 M .LE. 2 |
| `IERROR` | `2` | 2 N .LE. 2 |
| `IERROR` | `3` | 3 IDIMY .LT. M |
| `IERROR` | `4` | 4 NPEROD .LT. 0 or NPEROD .GT. 4 |
| `IERROR` | `5` | 5 MPEROD .LT. 0 or MPEROD .GT. 1 |
| `IERROR` | `6` | 6 A(I) .NE. C(1) or C(I) .NE. C(1) or B(I) .NE. B(1) for |
| `IERROR` | `1` | ,2,...,M. |
| `IERROR` | `7` | 7 A(1) .NE. 0 or C(M) .NE. 0 and MPEROD = 1 |

# Workspace and array requirements

- `A`: not a workspace argument
- `B`: not a workspace argument
- `C`: not a workspace argument
- `IDIMY`: not a workspace argument
- `Y`: not a workspace argument
- `W`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::genbun`
- Original SLATEC routine: `GENBUN`
- Native symbol: `genbun_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [GENBUN](https://www.netlib.org/slatec/fishfft/genbun.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
