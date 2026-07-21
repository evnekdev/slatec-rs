# Purpose

CPOCO factors a complex Hermitian positive definite matrix and estimates the condition of the matrix. If RCOND is not needed, CPOFA is slightly faster.

# Description

This canonical unsafe binding exposes original SLATEC routine `CPOCO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPOCO](https://www.netlib.org/slatec/lin/cpoco.f).

# Arguments

## 1. `A`

input `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). B , follow CPOCO by CPOSL. To compute  INVERSE(A)*C , follow CPOCO by CPOSL. To compute  DETERMINANT(A) , follow CPOCO by CPODI. To compute  INVERSE(A) , follow CPOCO by CPODI. On Entry COMPLEX(LDA, N) the Hermitian matrix to be factored.  Only the diagonal and upper triangle are used. CTRANS(R)*R where  CTRANS(R)  is the conjugate CTRANS(R)*R where  CTRANS(R)  is the conjugate transpose.  The strict lower triangle is unaltered. transpose.  The strict lower triangle is unaltered. If  INFO .NE. 0 , the factorization is not complete. If  INFO .NE. 0 , the factorization is not complete. B , relative perturbations in  A  and  B  of size  EPSILON  may cause relative perturbations in  X  of size  EPSILON/RCOND . If  RCOND  is so small that the logical expression 1.0 + RCOND .EQ. 1.0 is true, then  A  may be singular to working precision.  In particular,  RCOND  is zero  if exact singularity is detected or the estimate work vector whose contents are usually unimportant. If  A  is close to a singular matrix, then  Z  is an approximate null vector in the sense that RCOND*NORM(A)*NORM(Z) . not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `RCOND`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. REAL an estimate of the reciprocal condition of  A . is unchanged. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `Z`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 1; dimensions (*). COMPLEX(N) RCOND*NORM(A)*NORM(Z) . is unchanged. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is unchanged. is unchanged. INTEGER = 0  for normal return. = K  signals an error condition.  The leading minor of order  K  is not positive definite. not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `RCOND`: not a workspace argument
- `Z`: not a workspace argument
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cpoco`
- Original SLATEC routine: `CPOCO`
- Native symbol: `cpoco_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_f32,mut_complex32_array_rank1,mut_i32)`
- Exact Netlib source file: [CPOCO](https://www.netlib.org/slatec/lin/cpoco.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
