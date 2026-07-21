# Purpose

Find the zeros of the complex polynomial P(Z)= A(1)*Z**N + A(2)*Z**(N-1) +...+ A(N+1)

# Description

This canonical unsafe binding exposes original SLATEC routine `CPZERO`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CPZERO](https://www.netlib.org/slatec/src/cpzero.f).

# Arguments

## `IN`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

degree of P(Z).

## `A`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

complex vector containing coefficients of P(Z), coefficient of Z**(N+1-i).

## `R`

**Direction:** `input-output`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

N word complex vector containing initial estimates for zeros if these are known. Ith zero,.

## `T`

**Direction:** `input`. **Fortran type:** `COMPLEX`. **Rust ABI type:** `*mut crate::Complex32`. **Shape:** rank 1; dimensions (*).

4(N+1) word array used for temporary storage.

## `IFLG`

**Direction:** `output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

flag to indicate if initial estimates of zeros are input. If IFLG. EQ. 0, no estimates are input. NE. 0, the vector R contains estimates of the zeros WARNING ****** If estimates are input, they must be separated, that is, distinct or not repeated.

## `S`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

an N word array bound for R(I).

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `A`: not a workspace argument
- `R`: not a workspace argument
- `T`: not a workspace argument
- `S`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::roots::complex::cpzero`
- Original SLATEC routine: `CPZERO`
- Native symbol: `cpzero_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_i32,mut_f32_array_rank1)`
- Exact Netlib source file: [CPZERO](https://www.netlib.org/slatec/src/cpzero.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
