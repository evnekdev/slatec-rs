# Purpose

Subroutine POISTG solves the linear system of equations A(I)*X(I-1,J) + B(I)*X(I,J) + C(I)*X(I+1,J) + X(I,J-1) - 2.*X(I,J) + X(I,J+1) = Y(I,J) for I=1,2,...,M and J=1,2,...,N. The indices I+1 and I-1 are evaluated modulo M, i.e. X(0,J) = X(M,J) and X(M+1,J) = X(1,J), and X(I,0) may be equal to X(I,1) or -X(I,1) and X(I,N+1) may be equal to X(I,N) or -X(I,N) depending on an input parameter. * * * * * * * Parameter Description * * * * * * * * * *

# Description

This canonical unsafe binding exposes original SLATEC routine `POISTG`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [POISTG](https://www.netlib.org/slatec/fishfft/poistg.f).

# Arguments

## `NPEROD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Indicates the values which X(I,0) and X(I,N+1) are assumed to have. = 1 If X(I,0) = -X(I,1) and X(I,N+1) = -X(I,N) = 2 If X(I,0) = -X(I,1) and X(I,N+1) = X(I,N) = 3 If X(I,0) = X(I,1) and X(I,N+1) = X(I,N) = 4 If X(I,0) = X(I,1) and X(I,N+1) = -X(I,N).

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of unknowns in the J-direction. N must be greater than 2.

## `MPEROD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

= 0 If A(1) and C(M) are not zero = 1 If A(1) = C(M) = 0.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of unknowns in the I-direction. M must be greater than 2.

## `A`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

One-dimensional arrays of length M that specify the coefficients in the linear equations given above. If MPEROD = 0 the array elements must not depend on the index I, but must be constant. Specifically, the subroutine checks the following condition C(1). NE. C(1) or B(I). B(1) or C(I).

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

One-dimensional arrays of length M that specify the coefficients in the linear equations given above. If MPEROD = 0 the array elements must not depend on the index I, but must be constant. Specifically, the subroutine checks the following condition B(1).

## `C`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

One-dimensional arrays of length M that specify the coefficients in the linear equations given above. If MPEROD = 0 the array elements must not depend on the index I, but must be constant. Specifically, the subroutine checks the following condition C(1) for I = 1, 2,. , M.

## `IDIMY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The row (or first) dimension of the two-dimensional array Y as it appears in the program calling POISTG. This parameter is used to specify the variable dimension of Y. IDIMY must be at least M.

## `Y`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 2; dimensions (IDIMY, *).

A two-dimensional array that specifies the values of the right side of the linear system of equations given above. must be dimensioned at least M X N. Contains the solution X.

## `IERROR`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An error flag that indicates invalid input parameters. Except for number zero, a solution is not attempted. = 0 No error = 1 If M. LE. 2 = 2 If N. 2 = 3 IDIMY.

## `W`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional work array that must be provided by the user for work space. W may require up to 9M + 4N + M(INT(log2(N))) locations. The actual number of locations used is computed by POISTG and returned in location W(1). contains the required length of W.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error |
| `IERROR` | `1` | 1 If M .LE. 2 |
| `IERROR` | `2` | 2 If N .LE. 2 |
| `IERROR` | `3` | 3 IDIMY .LT. M |
| `IERROR` | `4` | 4 If NPEROD .LT. 1 or NPEROD .GT. 4 |
| `IERROR` | `5` | 5 If MPEROD .LT. 0 or MPEROD .GT. 1 |
| `IERROR` | `6` | 6 If MPEROD = 0 and |

# Workspace and array requirements

- `A`: not a workspace argument
- `B`: not a workspace argument
- `C`: not a workspace argument
- `IDIMY`: not a workspace argument
- `Y`: not a workspace argument
- `W`: A one-dimensional work array that must be provided by the user for work space. W may require up to 9M + 4N + M(INT(log2(N))) locations. The actual number of locations used is computed by POISTG and returned in location W(1). contains the required length of W.

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::poistg`
- Original SLATEC routine: `POISTG`
- Native symbol: `poistg_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [POISTG](https://www.netlib.org/slatec/fishfft/poistg.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
