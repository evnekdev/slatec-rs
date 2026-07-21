# Purpose

CQRSL applies the output of CQRDC to compute coordinate transformations, projections, and least squares solutions. For K .LE. MIN(N,P), let XK be the matrix XK = (X(JVPT(1)),X(JVPT(2)), ... ,X(JVPT(K))) formed from columns JVPT(1), ... ,JVPT(K) of the original N x P matrix X that was input to CQRDC (if no pivoting was done, XK consists of the first K columns of X in their original order). CQRDC produces a factored unitary matrix Q and an upper triangular matrix R such that XK = Q * (R) (0) This information is contained in coded form in the arrays X and QRAUX. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CQRSL`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CQRSL](https://www.netlib.org/slatec/lin/cqrsl.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 2; dimensions (LDX, *).

COMPLEX(LDX,P). contains the output of CQRDC.

## `LDX`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the leading dimension of the array X.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the number of rows of the matrix XK. It must have the same value as N in CQRDC.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is the number of columns of the matrix XK. K must not be greater than (N,P), where P is the same as in the calling sequence to CQRDC.

## `QRAUX`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(P). contains the auxiliary output from CQRDC.

## `Y`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N) contains an N-vector that is to be manipulated by CQRSL.

## `QY`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N). contains Q*Y, if its computation has been requested.

## `QTY`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N). contains CTRANS(Q)*Y, if its computation has been requested. Here CTRANS(Q) is the conjugate transpose of the matrix Q.

## `B`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(K) contains the solution of the least squares problem minimize NORM2(Y - XK*B), if its computation has been requested. (Note that if pivoting was requested in CQRDC, the J-th component of B will be associated with column JVPT(J) of the original matrix X that was input into CQRDC. ).

## `RSD`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N). contains the least squares residual Y - XK*B, if its computation has been requested. RSD is also the orthogonal projection of Y onto the orthogonal complement of the column space of XK.

## `XB`

**Direction:** `output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

COMPLEX(N). contains the least squares approximation XK*B, if its computation has been requested. XB is also the orthogonal projection of Y onto the column space of X.

## `JOB`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. specifies what is to be computed. JOB has the decimal expansion ABCDE, with the following meaning. If A. NE. 0, compute QY.

## `INFO`

**Direction:** `status-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

INTEGER. is zero unless the computation of B has been requested and R is exactly singular. In this case, INFO is the index of the first zero diagonal element of R and B is left unaltered. The parameters QY, QTY, B, RSD, and XB are not referenced if their computation is not requested and in this case can be replaced by dummy variables in the calling program. To save storage, the user may in some cases use the same array for different parameters in the calling sequence. A frequently occurring example is when one wishes to compute any of B, RSD, or XB and does not need Y or QTY.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Status and error values

`INFO` is a documented status output; its bounded argument contract states the available source semantics.

# Workspace and array requirements

- `X`: not a workspace argument
- `LDX`: not a workspace argument
- `QRAUX`: not a workspace argument
- `Y`: not a workspace argument
- `QY`: not a workspace argument
- `QTY`: not a workspace argument
- `B`: not a workspace argument
- `RSD`: not a workspace argument
- `XB`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cqrsl`
- Original SLATEC routine: `CQRSL`
- Native symbol: `cqrsl_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [CQRSL](https://www.netlib.org/slatec/lin/cqrsl.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
