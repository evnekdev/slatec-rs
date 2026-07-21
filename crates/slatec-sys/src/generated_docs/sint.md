# Purpose

Subroutine SINT computes the discrete Fourier sine transform of an odd sequence X(I). The transform is defined below at output parameter X. SINT is the unnormalized inverse of itself since a call of SINT followed by another call of SINT will multiply the input sequence

# Description

This canonical unsafe binding exposes original SLATEC routine `SINT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SINT](https://www.netlib.org/slatec/fishfft/sint.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. the length of the sequence to be transformed.  The method is most efficient when N+1 is the product of small primes. A call of SINT followed by another call of SINT will multiply the sequence X by 2*(N+1). Hence SINT is the unnormalized inverse of itself. the length of the sequence to be transformed.  The method is most efficient when N+1 is the product of small primes. A call of SINT followed by another call of SINT will multiply the sequence X by 2*(N+1). Hence SINT is the unnormalized inverse of itself. not applicable or not stated by selected source not a workspace argument

## 2. `X`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). by 2*(N+1). The array WSAVE which is used by subroutine SINT must be initialized by calling subroutine SINTI(N,WSAVE). an array which contains the sequence to be transformed 1,...,N the sum from K=1 to K=N A call of SINT followed by another call of SINT will multiply the sequence X by 2*(N+1). Hence SINT is the unnormalized inverse of itself. by 2*(N+1). The array WSAVE which is used by subroutine SINT must be initialized by calling subroutine SINTI(N,WSAVE). an array which contains the sequence to be transformed 1,...,N the sum from K=1 to K=N A call of SINT followed by another call of SINT will multiply the sequence X by 2*(N+1). Hence SINT is the unnormalized inverse of itself. not applicable or not stated by selected source not a workspace argument

## 3. `WSAVE`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). a work array with dimension at least INT(3.5*N+16) in the program that calls SINT.  The WSAVE array must be initialized by calling subroutine SINTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. contains initialization calculations which must not be destroyed between calls of SINT. a work array with dimension at least INT(3.5*N+16) in the program that calls SINT.  The WSAVE array must be initialized by calling subroutine SINTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. contains initialization calculations which must not be destroyed between calls of SINT. not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `X`: not a workspace argument
- `WSAVE`: a work array with dimension at least INT(3.5*N+16) in the program that calls SINT.  The WSAVE array must be initialized by calling subroutine SINTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. contains initialization calculations which must not be destroyed between calls of SINT.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::sint`
- Original SLATEC routine: `SINT`
- Native symbol: `sint_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SINT](https://www.netlib.org/slatec/fishfft/sint.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
