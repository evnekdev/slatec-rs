# Purpose

SPBCO factors a real symmetric positive definite matrix stored in band form and estimates the condition of the matrix. If RCOND is not needed, SPBFA is slightly faster. To solve A*X = B , follow SPBCO by SPBSL. To compute INVERSE(A)*C , follow SPBCO by SPBSL. To compute DETERMINANT(A) , follow SPBCO by SPBDI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SPBCO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SPBCO](https://www.netlib.org/slatec/lin/spbco.f).

# Arguments

## 1. `ABD`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). REAL(LDA, N) the matrix to be factored.  The columns of the upper triangle are stored in the columns of ABD and the diagonals of the upper triangle are stored in the rows of ABD .  See the comments below for details. an upper triangular matrix  R , stored in band form, so that  A = TRANS(R)*R . If  INFO .NE. 0 , the factorization is not complete. A(I,J) 10    CONTINUE 20 CONTINUE This uses  M + 1  rows of  A , except for the  M by M upper left triangle, which is ignored. Example:  If the original matrix is 11 12 13  0  0  0 12 22 23 24  0  0 13 23 33 34 35  0 0 24 34 44 45 46 0  0 35 45 55 56 0  0  0 46 56 66 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  ABD . must be .GE. M + 1 . INTEGER the leading dimension of the array  ABD . must be .GE. M + 1 . INTEGER the leading dimension of the array  ABD . must be .GE. M + 1 . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . 6 , M = 2  and  ABD  should contain * 13 24 35 46 12 23 34 45 56 11 22 33 44 55 66 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `M`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the number of diagonals above the main diagonal. 0 .LE. M .LT. N . On Return (band width above diagonal) DO 20 J = 1, N I1 = MAX(1, J-M) DO 10 I = I1, J K = I-J+M+1 not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `RCOND`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. REAL an estimate of the reciprocal condition of  A . For the system  A*X = B , relative perturbations in  A  and  B  of size  EPSILON  may cause relative perturbations in  X  of size  EPSILON/RCOND . If  RCOND  is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then  A  may be singular to working precision.  In particular,  RCOND  is zero  if exact singularity is detected or the estimate is unchanged. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `Z`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) a work vector whose contents are usually unimportant. If  A  is singular to working precision, then  Z  is an approximate null vector in the sense that RCOND*NORM(A)*NORM(Z) . is unchanged. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is unchanged. is unchanged. INTEGER = 0  for normal return. = K  signals an error condition.  The leading minor of order  K  is not positive definite. Band Storage If  A  is a symmetric positive definite band matrix, the following program segment will set up the input. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `ABD`: not a workspace argument
- `LDA`: not a workspace argument
- `N`: not a workspace argument
- `M`: not a workspace argument
- `RCOND`: not a workspace argument
- `Z`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::banded::spbco`
- Original SLATEC routine: `SPBCO`
- Native symbol: `spbco_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32,mut_f32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [SPBCO](https://www.netlib.org/slatec/lin/spbco.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
