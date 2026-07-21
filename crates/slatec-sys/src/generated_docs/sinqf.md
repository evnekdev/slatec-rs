# Purpose

Subroutine SINQF computes the fast Fourier transform of quarter wave data. That is, SINQF computes the coefficients in a sine series representation with only odd wave numbers. The transform is defined below at output parameter X. SINQB is the unnormalized inverse of SINQF since a call of SINQF followed by a call of SINQB will multiply the input sequence X by 4*N. The array WSAVE which is used by subroutine SINQF must be initialized by calling subroutine SINQI(N,WSAVE).

# Description

This canonical unsafe binding exposes original SLATEC routine `SINQF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SINQF](https://www.netlib.org/slatec/fishfft/sinqf.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the length of the array X to be transformed. The method is most efficient when N is a product of small primes.

## `X`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

an array which contains the sequence to be transformed For I=1,. ,N (-1)**(I-1)*X(N) + the sum from K=1 to K=N-1 of 2*X(K)*SIN((2*I-1)*K*PI/(2*N)) A call of SINQF followed by a call of SINQB will multiply the sequence X by 4*N. Therefore SINQB is the unnormalized inverse of SINQF.

## `WSAVE`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a work array which must be dimensioned at least 3*N+15 in the program that calls SINQF. The WSAVE array must be initialized by calling subroutine SINQI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. contains initialization calculations which must not be destroyed between calls of SINQF or SINQB.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `WSAVE`: a work array which must be dimensioned at least 3*N+15 in the program that calls SINQF. The WSAVE array must be initialized by calling subroutine SINQI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. contains initialization calculations which must not be destroyed between calls of SINQF or SINQB.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::sinqf`
- Original SLATEC routine: `SINQF`
- Native symbol: `sinqf_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SINQF](https://www.netlib.org/slatec/fishfft/sinqf.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
