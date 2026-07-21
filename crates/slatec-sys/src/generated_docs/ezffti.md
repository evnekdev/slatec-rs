# Purpose

Subroutine EZFFTI initializes the work array WSAVE which is used in both EZFFTF and EZFFTB. The prime factorization of N together with a tabulation of the trigonometric functions are computed and stored in WSAVE. Input Parameter

# Description

This canonical unsafe binding exposes original SLATEC routine `EZFFTI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [EZFFTI](https://www.netlib.org/slatec/fishfft/ezffti.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. the length of the sequence to be transformed. Output Parameter the length of the sequence to be transformed. Output Parameter not applicable or not stated by selected source not a workspace argument

## 2. `WSAVE`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). a work array which must be dimensioned at least 3*N+15. The same work array can be used for both EZFFTF and EZFFTB as long as N remains unchanged.  Different WSAVE arrays are required for different values of N. a work array which must be dimensioned at least 3*N+15. The same work array can be used for both EZFFTF and EZFFTB as long as N remains unchanged.  Different WSAVE arrays are required for different values of N. not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `WSAVE`: a work array which must be dimensioned at least 3*N+15. The same work array can be used for both EZFFTF and EZFFTB as long as N remains unchanged.  Different WSAVE arrays are required for different values of N.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::ezffti`
- Original SLATEC routine: `EZFFTI`
- Native symbol: `ezffti_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [EZFFTI](https://www.netlib.org/slatec/fishfft/ezffti.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
