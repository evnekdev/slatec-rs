# Purpose

Subroutine EZFFTF computes the Fourier coefficients of a real periodic sequence (Fourier analysis). The transform is defined below at Output Parameters AZERO, A and B. EZFFTF is a simplified but slower version of RFFTF.

# Description

This canonical unsafe binding exposes original SLATEC routine `EZFFTF`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [EZFFTF](https://www.netlib.org/slatec/fishfft/ezfftf.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the length of the array R to be transformed. The method is most efficient when N is the product of small primes.

## `R`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a real array of length N which contains the sequence to be transformed. R is not destroyed.

## `AZERO`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

the sum from I=1 to I=N of R(I)/N.

## `A`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

for N even B(N/2)=0. and A(N/2) is the sum from I=1 to I=N of (-1)**(I-1)*R(I)/N for N even define KMAX=N/2-1 for N odd define KMAX=(N-1)/2 then for K=1,. ,KMAX A(K) equals the sum from I=1 to I=N of 2. /N*R(I)*COS(K*(I-1)*2*PI/N) B(K) equals the sum from I=1 to I=N of 2. /N*R(I)*SIN(K*(I-1)*2*PI/N).

## `B`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

for N even B(N/2)=0. and A(N/2) is the sum from I=1 to I=N of (-1)**(I-1)*R(I)/N for N even define KMAX=N/2-1 for N odd define KMAX=(N-1)/2 then for K=1,. ,KMAX A(K) equals the sum from I=1 to I=N of 2. /N*R(I)*COS(K*(I-1)*2*PI/N) B(K) equals the sum from I=1 to I=N of 2. /N*R(I)*SIN(K*(I-1)*2*PI/N).

## `WSAVE`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTF. The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `R`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `WSAVE`: a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTF. The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::ezfftf`
- Original SLATEC routine: `EZFFTF`
- Native symbol: `ezfftf_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [EZFFTF](https://www.netlib.org/slatec/fishfft/ezfftf.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
