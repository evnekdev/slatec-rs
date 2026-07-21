# Purpose

CSIFA factors a complex symmetric matrix by elimination with symmetric pivoting.

# Description

This canonical unsafe binding exposes original SLATEC routine `CSIFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CSIFA](https://www.netlib.org/slatec/lin/csifa.f).

# Arguments

## 1. `A`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). B , follow CSIFA by CSISL. To compute  INVERSE(A)*C , follow CSIFA by CSISL. To compute  DETERMINANT(A) , follow CSIFA by CSIDI. To compute  INVERSE(A) , follow CSIFA by CSIDI. On Entry COMPLEX(LDA,N) the symmetric matrix to be factored. Only the diagonal and upper triangle are used. a block diagonal matrix and the multipliers which were used to obtain it. U*D*TRANS(U) where  U  is a product of permutation and unit upper triangular matrices , TRANS(U) is the transpose of  U , and  D  is block diagonal with 1 by 1 and 2 by 2 blocks. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `KPVT`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) an integer vector of pivot indices. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 0  normal value. = K  if the K-th pivot block is singular.  This is not an error condition for this subroutine, but it does indicate that CSISL or CSIDI may divide by zero if called. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `A`: not a workspace argument
- `LDA`: not a workspace argument
- `N`: not a workspace argument
- `KPVT`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::csifa`
- Original SLATEC routine: `CSIFA`
- Native symbol: `csifa_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32_array_rank1,mut_i32)`
- Exact Netlib source file: [CSIFA](https://www.netlib.org/slatec/lin/csifa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
