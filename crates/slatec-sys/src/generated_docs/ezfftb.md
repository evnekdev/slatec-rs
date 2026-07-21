# Purpose

Subroutine EZFFTB computes a real periodic sequence from its Fourier coefficients (Fourier synthesis). The transform is defined below at Output Parameter R. EZFFTB is a simplified but slower version of RFFTB.

# Description

This canonical unsafe binding exposes original SLATEC routine `EZFFTB`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [EZFFTB](https://www.netlib.org/slatec/fishfft/ezfftb.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. is most efficient when N is the product of small primes. are required. are required. 1)/2 locations are required 1)/2 locations are required N/2 1)/2 1)/2 Then for I=1,...,N Then for I=1,...,N where where ALPHA(K) = SQRT(A(K)*A(K)+B(K)*B(K)) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 2. `R`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). is most efficient when N is the product of small primes. N/2 AZERO plus the sum from K=1 to K=KMAX of KMAX to K=KMAX of 1 to K=KMAX of not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `AZERO`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. the constant Fourier coefficient 1 to K=KMAX of not stated by selected source not applicable or not stated by selected source not a workspace argument

## 4. `A`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the remaining Fourier coefficients. These arrays are not destroyed. The length of these arrays depends on whether N is even or odd. must be dimensioned at least 3*N+15 in the program that calls EZFFTB.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. 1)*2*PI/N)+B(K)*SIN(K*(I-1)*2*PI/N) Complex Notation ************************** For J=1,...,N B(K))   for K=1,...,KMAX C(-K) = CONJG(C(K)) C(0) = AZERO and I=SQRT(-1) Amplitude - Phase Notation *********************** For I=1,...,N contain the remaining Fourier coefficients. These arrays are not destroyed. The length of these arrays depends on whether N is even or odd. must be dimensioned at least 3*N+15 in the program that calls EZFFTB.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. 1)*2*PI/N)+B(K)*SIN(K*(I-1)*2*PI/N) Complex Notation ************************** For J=1,...,N B(K))   for K=1,...,KMAX C(-K) = CONJG(C(K)) C(0) = AZERO and I=SQRT(-1) Amplitude - Phase Notation *********************** For I=1,...,N not applicable or not stated by selected source not a workspace argument

## 5. `B`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contain the remaining Fourier coefficients. These arrays are not destroyed. The length of these arrays depends on whether N is even or odd. contain the remaining Fourier coefficients. These arrays are not destroyed. The length of these arrays depends on whether N is even or odd. not applicable or not stated by selected source not a workspace argument

## 6. `WSAVE`

workspace `workspace` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). must be dimensioned at least 3*N+15 in the program that calls EZFFTB.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. must be dimensioned at least 3*N+15 in the program that calls EZFFTB.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `R`: not a workspace argument
- `AZERO`: not a workspace argument
- `A`: not a workspace argument
- `B`: not a workspace argument
- `WSAVE`: must be dimensioned at least 3*N+15 in the program that calls EZFFTB.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::ezfftb`
- Original SLATEC routine: `EZFFTB`
- Native symbol: `ezfftb_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [EZFFTB](https://www.netlib.org/slatec/fishfft/ezfftb.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
