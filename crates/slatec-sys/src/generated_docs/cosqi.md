# Purpose

Subroutine COSQI initializes the work array WSAVE which is used in both COSQF1 and COSQB1. The prime factorization of N together with a tabulation of the trigonometric functions are computed and stored in WSAVE. Input Parameter

# Description

This canonical unsafe binding exposes original SLATEC routine `COSQI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [COSQI](https://www.netlib.org/slatec/fishfft/cosqi.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. the length of the array to be transformed.  The method is most efficient when N is a product of small primes. Output Parameter the length of the array to be transformed.  The method is most efficient when N is a product of small primes. Output Parameter not applicable or not stated by selected source not a workspace argument

## 2. `WSAVE`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). a work array which must be dimensioned at least 3*N+15. The same work array can be used for both COSQF1 and COSQB1 as long as N remains unchanged.  Different WSAVE arrays are required for different values of N.  The contents of must not be changed between calls of COSQF1 or COSQB1. a work array which must be dimensioned at least 3*N+15. The same work array can be used for both COSQF1 and COSQB1 as long as N remains unchanged.  Different WSAVE arrays are required for different values of N.  The contents of must not be changed between calls of COSQF1 or COSQB1. not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `WSAVE`: a work array which must be dimensioned at least 3*N+15. The same work array can be used for both COSQF1 and COSQB1 as long as N remains unchanged.  Different WSAVE arrays are required for different values of N.  The contents of must not be changed between calls of COSQF1 or COSQB1.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::cosqi`
- Original SLATEC routine: `COSQI`
- Native symbol: `cosqi_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [COSQI](https://www.netlib.org/slatec/fishfft/cosqi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
