# Purpose

SPOFA factors a real symmetric positive definite matrix. SPOFA is usually called by SPOCO, but it can be called directly with a saving in time if RCOND is not needed. (Time for SPOCO) = (1 + 18/N)*(Time for SPOFA) . On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SPOFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SPOFA](https://www.netlib.org/slatec/lin/spofa.f).

# Arguments

## 1. `A`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). REAL(LDA, N) the symmetric matrix to be factored.  Only the diagonal and upper triangle are used. TRANS(R)*R TRANS(R)*R where  TRANS(R)  is the transpose. where  TRANS(R)  is the transpose. The strict lower triangle is unaltered. The strict lower triangle is unaltered. If  INFO .NE. 0 , the factorization is not complete. If  INFO .NE. 0 , the factorization is not complete. not stated by selected source not applicable or not stated by selected source not a workspace argument

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

- Canonical Rust path: `slatec_sys::linear_algebra::dense::spofa`
- Original SLATEC routine: `SPOFA`
- Native symbol: `spofa_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32)`
- Exact Netlib source file: [SPOFA](https://www.netlib.org/slatec/lin/spofa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
