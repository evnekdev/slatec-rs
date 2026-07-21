# Purpose

Subroutine CBLKTR is a complex version of subroutine BLKTRI. Both subroutines solve a system of linear equations of the form AN(J)*X(I,J-1) + AM(I)*X(I-1,J) + (BN(J)+BM(I))*X(I,J) + CN(J)*X(I,J+1) + CM(I)*X(I+1,J) = Y(I,J) For I = 1,2,...,M and J = 1,2,...,N. I+1 and I-1 are evaluated modulo M and J+1 and J-1 modulo N, i.e., X(I,0) = X(I,N), X(I,N+1) = X(I,1), X(0,J) = X(M,J), X(M+1,J) = X(1,J). These equations usually result from the discretization of separable elliptic equations. Boundary conditions may be Dirichlet, Neumann, or periodic.

# Description

This canonical unsafe binding exposes original SLATEC routine `CBLKTR`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CBLKTR](https://www.netlib.org/slatec/fishfft/cblktr.f).

# Arguments

## `IFLG`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

= 0 Initialization only. Certain quantities that depend on NP,.

## `NP`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

= 0 If AN(1) and CN(N) are not zero, which corresponds to periodic boundary conditions. = 1 If AN(1) and CN(N) are zero.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

computed and stored in the work array W. = 1 The quantities that were computed in the initialization are used to obtain the solution X(I,J). NOTE A call with IFLG=0 takes approximately one half the time time as a call with IFLG = 1. However, the initialization does not have to be repeated unless NP, N, AN, BN, or CN change. The number of unknowns in the J-direction. N must be greater than 4.

## `AN`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

computed and stored in the work array W. = 1 The quantities that were computed in the initialization are used to obtain the solution X(I,J). NOTE A call with IFLG=0 takes approximately one half the time time as a call with IFLG = 1. However, the initialization does not have to be repeated unless NP, N, AN, BN, or CN change. one-dimensional arrays of length N that specify the coefficients in the linear equations given above.

## `BN`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

computed and stored in the work array W. = 1 The quantities that were computed in the initialization are used to obtain the solution X(I,J). NOTE A call with IFLG=0 takes approximately one half the time time as a call with IFLG = 1. However, the initialization does not have to be repeated unless NP, N, AN, BN, or CN change. one-dimensional arrays of length N that specify the coefficients in the linear equations given above.

## `CN`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

computed and stored in the work array W. = 1 The quantities that were computed in the initialization are used to obtain the solution X(I,J). NOTE A call with IFLG=0 takes approximately one half the time time as a call with IFLG = 1. However, the initialization does not have to be repeated unless NP, N, AN, BN, or CN change. one-dimensional arrays of length N that specify the coefficients in the linear equations given above.

## `MP`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Input M-direction coupling selector. `MP=0` requests the periodic coefficient case and requires the endpoint off-diagonal coefficients to be nonzero; `MP=1` selects the noncyclic endpoint case where those endpoint coefficients are zero.

## `M`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The number of unknowns in the I-direction. M must be greater than 4.

## `AM`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

Complex one-dimensional arrays of length M that specify the coefficients in the linear equations given above.

## `BM`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

Complex one-dimensional arrays of length M that specify the coefficients in the linear equations given above.

## `CM`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

Complex one-dimensional arrays of length M that specify the coefficients in the linear equations given above.

## `IDIMY`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

The row (or first) dimension of the two-dimensional array Y as it appears in the program calling BLKTRI. This parameter is used to specify the variable dimension of Y. IDIMY must be at least M.

## `Y`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (IDIMY, *).

A complex two-dimensional array that specifies the values of the right side of the linear system of equations given above. must be dimensioned Y(IDIMY,N) with IDIMY. GE. M. Contains the solution X.

## `IERROR`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Writable status output. `0` means success; `1` means `M < 5`; `2` means `N < 5`; `3` means `IDIMY < M`; `4` reports a coefficient-array failure; and `5` reports an invalid negative product in the tridiagonal coefficient condition. Except for `0`, no solution is produced.

## `W`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

A one-dimensional array that must be provided by the user for must have dimension (K-2)*L+K+5+MAX(2N,12M) If NP=0 define K=INT(log2(N-1))+1 and set L=2**(K+1) then must have dimension (K-2)*L+K+5+2N+MAX(2N,12M) IMPORTANT** For purposes of checking, the required dimension of W is computed by BLKTRI and stored in W(1) in floating point format. Contains intermediate values that must not be destroyed if CBLKTR will be called again with IFLG=1. W(1) contains the number of locations required by W in floating point format.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `0` | 0 No error. |
| `IERROR` | `1` | 1 M is less than 5. |
| `IERROR` | `2` | 2 N is less than 5. |
| `IERROR` | `3` | 3 IDIMY is less than M. |
| `IERROR` | `4` | 4 BLKTRI failed while computing results that depend on the coefficient arrays AN, BN, CN. Check these arrays. |
| `IERROR` | `5` | 5 AN(J)*CN(J-1) is less than 0 for some J. Possible reasons for this condition are 1. The arrays AN and CN are not correct. 2. Too large a grid spacing was used in the discretization of the elliptic equation. 3. The linear equations resulted from a partial differential equation which was not elliptic. |

# Workspace and array requirements

- `AN`: computed and stored in the work array W. = 1 The quantities that were computed in the initialization are used to obtain the solution X(I,J). NOTE A call with IFLG=0 takes approximately one half the time time as a call with IFLG = 1. However, the initialization does not have to be repeated unless NP, N, AN, BN, or CN change. one-dimensional arrays of length N that specify the coefficients in the linear equations given above.
- `BN`: computed and stored in the work array W. = 1 The quantities that were computed in the initialization are used to obtain the solution X(I,J). NOTE A call with IFLG=0 takes approximately one half the time time as a call with IFLG = 1. However, the initialization does not have to be repeated unless NP, N, AN, BN, or CN change. one-dimensional arrays of length N that specify the coefficients in the linear equations given above.
- `CN`: computed and stored in the work array W. = 1 The quantities that were computed in the initialization are used to obtain the solution X(I,J). NOTE A call with IFLG=0 takes approximately one half the time time as a call with IFLG = 1. However, the initialization does not have to be repeated unless NP, N, AN, BN, or CN change. one-dimensional arrays of length N that specify the coefficients in the linear equations given above.
- `AM`: not a workspace argument
- `BM`: not a workspace argument
- `CM`: not a workspace argument
- `IDIMY`: not a workspace argument
- `Y`: not a workspace argument
- `W`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::pde::fishpack::complex::cblktr`
- Original SLATEC routine: `CBLKTR`
- Native symbol: `cblktr_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_array_rank1,mut_f32_array_rank1,mut_f32_array_rank1,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank2,mut_i32,mut_f32_array_rank1)`
- Exact Netlib source file: [CBLKTR](https://www.netlib.org/slatec/fishfft/cblktr.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
