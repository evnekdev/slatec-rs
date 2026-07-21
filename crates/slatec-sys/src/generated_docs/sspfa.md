# Purpose

SSPFA factors a real symmetric matrix stored in packed form by elimination with symmetric pivoting. To solve A*X = B , follow SSPFA by SSPSL. To compute INVERSE(A)*C , follow SSPFA by SSPSL. To compute DETERMINANT(A) , follow SSPFA by SSPDI. To compute INERTIA(A) , follow SSPFA by SSPDI. To compute INVERSE(A) , follow SSPFA by SSPDI. On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SSPFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SSPFA](https://www.netlib.org/slatec/lin/sspfa.f).

# Arguments

## 1. `AP`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL (N*(N+1)/2) the packed form of a symmetric matrix  A .  The columns of the upper triangle are stored sequentially in a one-dimensional array of length  N*(N+1)/2 . See comments below for details. a block diagonal matrix and the multipliers which were used to obtain it stored in packed form. The factorization can be written  A = U*D*TRANS(U) where  U  is a product of permutation and unit upper triangular matrices , TRANS(U) is the transpose of  U , and  D  is block diagonal with 1 by 1 and 2 by 2 blocks. A(I,J) 10    CONTINUE 20 CONTINUE REAL (N*(N+1)/2) the packed form of a symmetric matrix  A .  The columns of the upper triangle are stored sequentially in a one-dimensional array of length  N*(N+1)/2 . See comments below for details. a block diagonal matrix and the multipliers which were used to obtain it stored in packed form. The factorization can be written  A = U*D*TRANS(U) where  U  is a product of permutation and unit upper triangular matrices , TRANS(U) is the transpose of  U , and  D  is block diagonal with 1 by 1 and 2 by 2 blocks. A(I,J) 10    CONTINUE 20 CONTINUE not applicable or not stated by selected source not a workspace argument

## 2. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `KPVT`

output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) an integer vector of pivot indices. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 0  normal value. = K  if the K-th pivot block is singular.  This is not an error condition for this subroutine, but it does indicate that SSPSL or SSPDI may divide by zero if called. Packed Storage The following program segment will pack the upper triangle of a symmetric matrix. K = 0 DO 20 J = 1, N DO 10 I = 1, J K = K + 1 not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::packed::sspfa`
- Original SLATEC routine: `SSPFA`
- Native symbol: `sspfa_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [SSPFA](https://www.netlib.org/slatec/lin/sspfa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
