# Purpose

CSPCO factors a complex symmetric matrix stored in packed form by elimination with symmetric pivoting and estimates the condition of the matrix. If RCOND is not needed, CSPFA is slightly faster. To solve A*X = B , follow CSPCO by CSPSL. To compute INVERSE(A)*C , follow CSPCO by CSPSL. To compute INVERSE(A) , follow CSPCO by CSPDI. To compute DETERMINANT(A) , follow CSPCO by CSPDI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CSPCO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSPCO](https://www.netlib.org/slatec/lin/cspco.f).

# Arguments

## 1. `AP`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX (N*(N+1)/2) the packed form of a symmetric matrix  A .  The columns of the upper triangle are stored sequentially in a one-dimensional array of length  N*(N+1)/2 . See comments below for details. a block diagonal matrix and the multipliers which were used to obtain it stored in packed form. The factorization can be written  A = U*D*TRANS(U) where  U  is a product of permutation and unit upper triangular matrices , TRANS(U) is the transpose of  U , and  D  is block diagonal with 1 by 1 and 2 by 2 blocks. A(I,J) 10    CONTINUE 20 CONTINUE COMPLEX (N*(N+1)/2) the packed form of a symmetric matrix  A .  The columns of the upper triangle are stored sequentially in a one-dimensional array of length  N*(N+1)/2 . See comments below for details. a block diagonal matrix and the multipliers which were used to obtain it stored in packed form. The factorization can be written  A = U*D*TRANS(U) where  U  is a product of permutation and unit upper triangular matrices , TRANS(U) is the transpose of  U , and  D  is block diagonal with 1 by 1 and 2 by 2 blocks. A(I,J) 10    CONTINUE 20 CONTINUE not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `KPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) an integer vector of pivot indices. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `RCOND`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. REAL an estimate of the reciprocal condition of  A . For the system  A*X = B , relative perturbations in  A  and  B  of size  EPSILON  may cause relative perturbations in  X  of size  EPSILON/RCOND . If  RCOND  is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then  A  may be singular to working precision.  In particular,  RCOND  is zero  if exact singularity is detected or the estimate underflows. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `Z`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) a work vector whose contents are usually unimportant. If  A  is close to a singular matrix, then  Z  is an approximate null vector in the sense that RCOND*NORM(A)*NORM(Z) . Packed Storage The following program segment will pack the upper triangle of a symmetric matrix. K = 0 DO 20 J = 1, N DO 10 I = 1, J K = K + 1 not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `AP`: not a workspace argument
- `N`: not a workspace argument
- `KPVT`: not a workspace argument
- `RCOND`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::complex::cspco`
- Original SLATEC routine: `CSPCO`
- Native symbol: `cspco_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank1,mut_i32,mut_i32_array_rank1,mut_f32,mut_complex32_array_rank1)`
- Exact Netlib source file: [CSPCO](https://www.netlib.org/slatec/lin/cspco.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
