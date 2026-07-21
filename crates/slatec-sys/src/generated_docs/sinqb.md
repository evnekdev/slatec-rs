# Purpose

Subroutine SINQB computes the fast Fourier transform of quarter wave data. That is, SINQB computes a sequence from its representation in terms of a sine series with odd wave numbers. the transform is defined below at output parameter X. SINQF is the unnormalized inverse of SINQB since a call of SINQB followed by a call of SINQF will multiply the input sequence X by 4*N. The array WSAVE which is used by subroutine SINQB must be initialized by calling subroutine SINQI(N,WSAVE).

# Description

This canonical unsafe binding exposes original SLATEC routine `SINQB`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [SINQB](https://www.netlib.org/slatec/fishfft/sinqb.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the length of the array X to be transformed. The method is most efficient when N is a product of small primes.

## `X`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

an array which contains the sequence to be transformed For I=1,. ,N the sum from K=1 to K=N of 4*X(K)*SIN((2*K-1)*I*PI/(2*N)) a call of SINQB followed by a call of SINQF will multiply the sequence X by 4*N. Therefore SINQF is the unnormalized inverse of SINQB.

## `WSAVE`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a work array which must be dimensioned at least 3*N+15 in the program that calls SINQB. The WSAVE array must be initialized by calling subroutine SINQI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. contains initialization calculations which must not be destroyed between calls of SINQB or SINQF.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `WSAVE`: a work array which must be dimensioned at least 3*N+15 in the program that calls SINQB. The WSAVE array must be initialized by calling subroutine SINQI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. contains initialization calculations which must not be destroyed between calls of SINQB or SINQF.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::sinqb`
- Original SLATEC routine: `SINQB`
- Native symbol: `sinqb_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [SINQB](https://www.netlib.org/slatec/fishfft/sinqb.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
