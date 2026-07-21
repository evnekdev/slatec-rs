# Purpose

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBINTK is the SPLINT routine of the reference. DBINTK produces the B-spline coefficients, BCOEF, of the B-spline of order K with knots T(I), I=1,...,N+K, which takes on the value Y(I) at X(I), I=1,...,N. The spline or any of its derivatives can be evaluated by calls to DBVALU. The I-th equation of the linear system A*BCOEF = B for the coefficients of the interpolant enforces interpolation at X(I), I=1,...,N. Hence, B(I) = Y(I), for all I, and A is a band matrix with 2K-1 bands if A is invertible. The matrix A is generated row by row and stored, diagonal by diagonal, in the rows of Q, with the main diagonal going into row K. The banded system is then solved by a call to DBNFAC (which constructs the triangular factorization for A and stores it again in Q), followed by a call to DBNSLV (which then obtains the solution BCOEF by substitution). DBNFAC does no pivoting, since the total positivity of the matrix A makes this unnecessary. The linear system to be solved is (theoretically) invertible if and only if

# Description

This canonical unsafe binding exposes original SLATEC routine `DBINTK`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DBINTK](https://www.netlib.org/slatec/src/dbintk.f).

# Arguments

## `X`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

vector of length N containing data point abscissa in strictly increasing order.

## `Y`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

corresponding vector of length N containing data point ordinates.

## `T`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

LT. X(I). T(I+K), for all I. Equality is permitted on the left for I=1 and on the right for I=N when K knots are used at X(1) or X(N). Otherwise, violation of this condition is certain to lead to an error. knot vector of length N+K Since T(1),.

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

number of data points, N. GE. K.

## `K`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

order of the spline, K. GE. 1 Output BCOEF,Q,WORK are double precision.

## `BCOEF`

**Direction:** `output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

a vector of length N containing the B-spline coefficients.

## `Q`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

a work vector of length (2*K-1)*N, containing the triangular factorization of the coefficient matrix of the linear system being solved. The coefficients for the interpolant of an additional data set (X(I),YY(I)), I=1,. ,N with the same abscissa can be obtained by loading YY into BCOEF and then executing CALL DBNSLV (Q,2K-1,N,K-1,K-1,BCOEF). a work vector of length (2*K-1)*N, containing the triangular factorization of the coefficient matrix of the linear system being solved. The coefficients for the interpolant of an additional data set (X(I),YY(I)), I=1,...,N with the same abscissa can be obtained by loading YY into BCOEF and then executing CALL DBNSLV (Q,2K-1,N,K-1,K-1,BCOEF)

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

work vector of length 2*K.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `Y`: not a workspace argument
- `T`: not a workspace argument
- `BCOEF`: not a workspace argument
- `Q`: a work vector of length (2*K-1)*N, containing the triangular factorization of the coefficient matrix of the linear system being solved. The coefficients for the interpolant of an additional data set (X(I),YY(I)), I=1,...,N with the same abscissa can be obtained by loading YY into BCOEF and then executing CALL DBNSLV (Q,2K-1,N,K-1,K-1,BCOEF)
- `WORK`: work vector of length 2*K

# ABI notes

- Canonical Rust path: `slatec_sys::interpolation::dbintk`
- Original SLATEC routine: `DBINTK`
- Native symbol: `dbintk_`
- ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1)`
- Exact Netlib source file: [DBINTK](https://www.netlib.org/slatec/src/dbintk.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
