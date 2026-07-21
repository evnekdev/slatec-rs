# Purpose

SGECO factors a real matrix by Gaussian elimination and estimates the condition of the matrix. If RCOND is not needed, SGEFA is slightly faster.

# Description

This canonical unsafe binding exposes original SLATEC routine `SGECO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SGECO](https://www.netlib.org/slatec/lin/sgeco.f).

# Arguments

## 1. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). B , follow SGECO by SGESL. To compute  INVERSE(A)*C , follow SGECO by SGESL. To compute  DETERMINANT(A) , follow SGECO by SGEDI. To compute  INVERSE(A) , follow SGECO by SGEDI. On Entry REAL(LDA, N) the matrix to be factored. an upper triangular matrix and the multipliers which were used to obtain it. L*U , where L  is a product of permutation and unit lower triangular matrices and  U  is upper triangular. B , relative perturbations in  A  and  B  of size  EPSILON  may cause relative perturbations in  X  of size  EPSILON/RCOND . If  RCOND  is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then  A  may be singular to working precision.  In particular,  RCOND  is zero  if exact singularity is detected or the estimate underflows. work vector whose contents are usually unimportant. If  A  is close to a singular matrix, then  Z  is an approximate null vector in the sense that RCOND*NORM(A)*NORM(Z) . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IPVT`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) an integer vector of pivot indices. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `RCOND`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. REAL an estimate of the reciprocal condition of  A . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `Z`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). REAL(N) RCOND*NORM(A)*NORM(Z) . not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `IPVT`: not a workspace argument
- `RCOND`: not a workspace argument
- `Z`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::sgeco`
- Original SLATEC routine: `SGECO`
- Native symbol: `sgeco_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SGECO](https://www.netlib.org/slatec/lin/sgeco.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
