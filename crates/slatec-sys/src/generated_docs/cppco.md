# Purpose

CPPCO factors a complex Hermitian positive definite matrix stored in packed form and estimates the condition of the matrix. If RCOND is not needed, CPPFA is slightly faster. To solve A*X = B , follow CPPCO by CPPSL. To compute INVERSE(A)*C , follow CPPCO by CPPSL. To compute DETERMINANT(A) , follow CPPCO by CPPDI. To compute INVERSE(A) , follow CPPCO by CPPDI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CPPCO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPPCO](https://www.netlib.org/slatec/lin/cppco.f).

# Arguments

## 1. `AP`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX (N*(N+1)/2) the packed form of a Hermitian matrix  A .  The columns of the upper triangle are stored sequentially in a one-dimensional array of length  N*(N+1)/2 . See comments below for details. an upper triangular matrix  R , stored in packed form, so that  A = CTRANS(R)*R . If  INFO .NE. 0 , the factorization is not complete. A(I,J) 10    CONTINUE 20 CONTINUE COMPLEX (N*(N+1)/2) the packed form of a Hermitian matrix  A .  The columns of the upper triangle are stored sequentially in a one-dimensional array of length  N*(N+1)/2 . See comments below for details. an upper triangular matrix  R , stored in packed form, so that  A = CTRANS(R)*R . If  INFO .NE. 0 , the factorization is not complete. A(I,J) 10    CONTINUE 20 CONTINUE not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `RCOND`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. REAL an estimate of the reciprocal condition of  A . For the system  A*X = B , relative perturbations in  A  and  B  of size  EPSILON  may cause relative perturbations in  X  of size  EPSILON/RCOND . If  RCOND  is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then  A  may be singular to working precision.  In particular,  RCOND  is zero  if exact singularity is detected or the estimate is unchanged. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `Z`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) a work vector whose contents are usually unimportant. If  A  is singular to working precision, then  Z  is an approximate null vector in the sense that RCOND*NORM(A)*NORM(Z) . is unchanged. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is unchanged. is unchanged. INTEGER = 0  for normal return. = K  signals an error condition.  The leading minor of order  K  is not positive definite. Packed Storage The following program segment will pack the upper triangle of a Hermitian matrix. K = 0 DO 20 J = 1, N DO 10 I = 1, J K = K + 1 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `AP`: not a workspace argument
- `N`: not a workspace argument
- `RCOND`: not a workspace argument
- `Z`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::complex::cppco`
- Original SLATEC routine: `CPPCO`
- Native symbol: `cppco_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank1,mut_i32,mut_f32,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CPPCO](https://www.netlib.org/slatec/lin/cppco.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
