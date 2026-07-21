# Purpose

SGEFA factors a real matrix by Gaussian elimination. SGEFA is usually called by SGECO, but it can be called directly with a saving in time if RCOND is not needed. (Time for SGECO) = (1 + 9/N)*(Time for SGEFA) . On Entry

# Description

This canonical unsafe binding exposes original SLATEC routine `SGEFA`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SGEFA](https://www.netlib.org/slatec/lin/sgefa.f).

# Arguments

## 1. `A`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 2; dimensions (LDA, *). REAL(LDA, N) the matrix to be factored. an upper triangular matrix and the multipliers which were used to obtain it. L*U , where L  is a product of permutation and unit lower triangular matrices and  U  is upper triangular. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `LDA`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . INTEGER the leading dimension of the array  A . not a workspace argument

## 3. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER the order of the matrix  A . On Return not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `IPVT`

output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). INTEGER(N) an integer vector of pivot indices. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 5. `INFO`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. INTEGER = 0  normal value. = K  if  U(K,K) .EQ. 0.0 .  This is not an error condition for this subroutine, but it does indicate that SGESL or SGEDI will divide by zero if called.  Use  RCOND  in SGECO for a reliable indication of singularity. not stated by selected source not applicable or not stated by selected source not a workspace argument

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
- `INFO`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::linear_algebra::dense::sgefa`
- Original SLATEC routine: `SGEFA`
- Native symbol: `sgefa_`
- ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_i32)`
- Exact Netlib source file: [SGEFA](https://www.netlib.org/slatec/lin/sgefa.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
