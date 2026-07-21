# Purpose

Subroutine EZFFTB computes a real periodic sequence from its Fourier coefficients (Fourier synthesis). The transform is defined below at Output Parameter R. EZFFTB is a simplified but slower version of RFFTB.

# Description

This canonical unsafe binding exposes original SLATEC routine `EZFFTB`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [EZFFTB](https://www.netlib.org/slatec/fishfft/ezfftb.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the length of the output array R. The method is most efficient when N is the product of small primes.

## `R`

**Direction:** `output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

if N is even, define KMAX=N/2 if N is odd, define KMAX=(N-1)/2 Then for I=1,. ,N AZERO plus the sum from K=1 to K=KMAX of A(K)*COS(K*(I-1)*2*PI/N)+B(K)*SIN(K*(I-1)*2*PI/N) Complex Notation ************************** For J=1,. ,N R(J) equals the sum from K=-KMAX to K=KMAX of C(K)*EXP(I*K*(J-1)*2*PI/N) where C(K) =. 5*CMPLX(A(K),-B(K)) for K=1,. ,KMAX C(-K) = CONJG(C(K)) C(0) = AZERO and I=SQRT(-1) Amplitude - Phase Notation *********************** For I=1,. ,N R(I) equals AZERO plus the sum from K=1 to K=KMAX of ALPHA(K)*COS(K*(I-1)*2*PI/N+BETA(K)) ALPHA(K) = SQRT(A(K)*A(K)+B(K)*B(K)) COS(BETA(K))=A(K)/ALPHA(K) SIN(BETA(K))=-B(K)/ALPHA(K).

## `AZERO`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** scalar.

the constant Fourier coefficient.

## `A`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

arrays which contain the remaining Fourier coefficients. These arrays are not destroyed. The length of these arrays depends on whether N is even or odd. If N is even, N/2 locations are required. If N is odd, (N-1)/2 locations are required.

## `B`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

arrays which contain the remaining Fourier coefficients. These arrays are not destroyed. The length of these arrays depends on whether N is even or odd. If N is even, N/2 locations are required. If N is odd, (N-1)/2 locations are required.

## `WSAVE`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTB. The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `R`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `WSAVE`: a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTB. The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::ezfftb`
- Original SLATEC routine: `EZFFTB`
- Native symbol: `ezfftb_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [EZFFTB](https://www.netlib.org/slatec/fishfft/ezfftb.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
