# Purpose

Subroutine SINT computes the discrete Fourier sine transform of an odd sequence X(I). The transform is defined below at output parameter X. SINT is the unnormalized inverse of itself since a call of SINT followed by another call of SINT will multiply the input sequence X by 2*(N+1). The array WSAVE which is used by subroutine SINT must be initialized by calling subroutine SINTI(N,WSAVE).

# Description

This canonical unsafe binding exposes original SLATEC routine `SINT`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SINT](https://www.netlib.org/slatec/fishfft/sint.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the length of the sequence to be transformed. The method is most efficient when N+1 is the product of small primes.

## `X`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

an array which contains the sequence to be transformed For I=1,. ,N the sum from K=1 to K=N 2*X(K)*SIN(K*I*PI/(N+1)) A call of SINT followed by another call of SINT will multiply the sequence X by 2*(N+1). Hence SINT is the unnormalized inverse of itself.

## `WSAVE`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a work array with dimension at least INT(3. 5*N+16) in the program that calls SINT. The WSAVE array must be initialized by calling subroutine SINTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. contains initialization calculations which must not be destroyed between calls of SINT.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `WSAVE`: a work array with dimension at least INT(3. 5*N+16) in the program that calls SINT. The WSAVE array must be initialized by calling subroutine SINTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. contains initialization calculations which must not be destroyed between calls of SINT.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::sint`
- Original SLATEC routine: `SINT`
- Native symbol: `sint_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SINT](https://www.netlib.org/slatec/fishfft/sint.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
