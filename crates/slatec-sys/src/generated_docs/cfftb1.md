# Purpose

Subroutine CFFTB1 computes the backward complex discrete Fourier transform (the Fourier synthesis). Equivalently, CFFTB1 computes a complex periodic sequence from its Fourier coefficients. The transform is defined below at output parameter C. A call of CFFTF1 followed by a call of CFFTB1 will multiply the sequence by N. The arrays WA and IFAC which are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC).

# Description

This canonical unsafe binding exposes original SLATEC routine `CFFTB1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CFFTB1](https://www.netlib.org/slatec/fishfft/cfftb1.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the length of the complex sequence C. The method is more efficient when N is the product of small primes.

## `C`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a complex array of length N which contains the sequence For J=1,. ,N the sum from K=1,. ,N of C(K)*EXP(I*(J-1)*(K-1)*2*PI/N) where I=SQRT(-1) NOTE: WA and IFAC contain initialization calculations which must not be destroyed between calls of subroutine CFFTF1 or CFFTB1.

## `CH`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a real work array of length at least 2*N.

## `WA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a real work array which must be dimensioned at least 2*N.

## `IFAC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

an integer work array which must be dimensioned at least 15. The WA and IFAC arrays must be initialized by calling subroutine CFFTI1 (N, WA, IFAC), and different WA and IFAC arrays must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WA and IFAC arrays can be used by CFFTF1 and CFFTB1.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `C`: not a workspace argument
- `CH`: a real work array of length at least 2*N.
- `WA`: a real work array which must be dimensioned at least 2*N.
- `IFAC`: an integer work array which must be dimensioned at least 15. The WA and IFAC arrays must be initialized by calling subroutine CFFTI1 (N, WA, IFAC), and different WA and IFAC arrays must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WA and IFAC arrays can be used by CFFTF1 and CFFTB1.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::cfftb1`
- Original SLATEC routine: `CFFTB1`
- Native symbol: `cfftb1_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [CFFTB1](https://www.netlib.org/slatec/fishfft/cfftb1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
