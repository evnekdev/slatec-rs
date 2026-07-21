# Purpose

CPOFA factors a complex Hermitian positive definite matrix. CPOFA is usually called by CPOCO, but it can be called directly with a saving in time if RCOND is not needed. (Time for CPOCO) = (1 + 18/N)*(Time for CPOFA) . On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `CPOFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPOFA](https://www.netlib.org/slatec/lin/cpofa.f).

# Arguments

## 1. `A`

input-output `array` argument; Fortran declaration `COMPLEX`, Rust ABI type `*mut crate::Complex32`, and rank 2; dimensions (LDA, *). COMPLEX(LDA, N) the Hermitian matrix to be factored.  Only the diagonal and upper triangle are used. CTRANS(R)*R where  CTRANS(R)  is the conjugate CTRANS(R)*R where  CTRANS(R)  is the conjugate transpose.  The strict lower triangle is unaltered. transpose.  The strict lower triangle is unaltered. If  INFO .NE. 0 , the factorization is not complete. If  INFO .NE. 0 , the factorization is not complete. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 0  for normal return. = K  signals an error condition.  The leading minor of order  K  is not positive definite. not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cpofa`
- Original SLATEC routine: `CPOFA`
- Native symbol: `cpofa_`
- ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [CPOFA](https://www.netlib.org/slatec/lin/cpofa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
