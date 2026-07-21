# Purpose

Subroutine RFFTF1 computes the Fourier coefficients of a real periodic sequence (Fourier analysis). The transform is defined below at output parameter C.

# Description

This canonical unsafe binding exposes original SLATEC routine `RFFTF1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RFFTF1](https://www.netlib.org/slatec/fishfft/rfftf1.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. the length of the array R to be transformed.  The method is most efficient when N is a product of small primes. may change so long as different work arrays are provided. contains the sequence to be transformed. N/2; if N is odd set L = (N+1)/2 then for K = 2,...,L If N is even the sum from I = 1 to I = N of the length of the array R to be transformed.  The method is most efficient when N is a product of small primes. may change so long as different work arrays are provided. contains the sequence to be transformed. N/2; if N is odd set L = (N+1)/2 then for K = 2,...,L If N is even the sum from I = 1 to I = N of not applicable or not stated by selected source

## 2. `C`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the sequence to be transformed. the sum from I=1 to I=N of R(I) the sum from I=1 to I=N of R(I) 2) = the sum from I = 1 to I = N of 1)*(I-1)*2*PI/N) 1) = the sum from I = 1 to I = N of If N is even the sum from I = 1 to I = N of Notes:  This transform is unnormalized since a call of RFFTF1 followed by a call of RFFTB1 will multiply the input sequence by N. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `CH`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). a real work array of length at least N. a real work array of length at least N. not applicable or not stated by selected source

## 4. `WA`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are used by subroutine RFFTB1 must be initialized by calling subroutine RFFTI1. Input Arguments a real work array which must be dimensioned at least N. must be initialized by calling must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first.  The same WA and IFAC arrays can be used by RFFTF1 and RFFTB1. Output Argument contain initialization calculations which must not be destroyed between calls of subroutine RFFTF1 or RFFTB1. are used by subroutine RFFTB1 must be initialized by calling subroutine RFFTI1. Input Arguments a real work array which must be dimensioned at least N. must be initialized by calling must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first.  The same WA and IFAC arrays can be used by RFFTF1 and RFFTB1. Output Argument contain initialization calculations which must not be destroyed between calls of subroutine RFFTF1 or RFFTB1. not applicable or not stated by selected source

## 5. `IFAC`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). are used by subroutine RFFTB1 must be initialized by calling subroutine RFFTI1. Input Arguments an integer work array which must be dimensioned at least 15. must be initialized by calling must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first.  The same WA and IFAC arrays can be used by RFFTF1 and RFFTB1. Output Argument contain initialization calculations which must not be destroyed between calls of subroutine RFFTF1 or RFFTB1. are used by subroutine RFFTB1 must be initialized by calling subroutine RFFTI1. Input Arguments an integer work array which must be dimensioned at least 15. must be initialized by calling must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first.  The same WA and IFAC arrays can be used by RFFTF1 and RFFTB1. Output Argument contain initialization calculations which must not be destroyed between calls of subroutine RFFTF1 or RFFTB1. not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: the length of the array R to be transformed.  The method is most efficient when N is a product of small primes. may change so long as different work arrays are provided. contains the sequence to be transformed. N/2; if N is odd set L = (N+1)/2 then for K = 2,...,L If N is even the sum from I = 1 to I = N of
- `C`: not a workspace argument
- `CH`: a real work array of length at least N.
- `WA`: are used by subroutine RFFTB1 must be initialized by calling subroutine RFFTI1. Input Arguments a real work array which must be dimensioned at least N. must be initialized by calling must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first.  The same WA and IFAC arrays can be used by RFFTF1 and RFFTB1. Output Argument contain initialization calculations which must not be destroyed between calls of subroutine RFFTF1 or RFFTB1.
- `IFAC`: are used by subroutine RFFTB1 must be initialized by calling subroutine RFFTI1. Input Arguments an integer work array which must be dimensioned at least 15. must be initialized by calling must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first.  The same WA and IFAC arrays can be used by RFFTF1 and RFFTB1. Output Argument contain initialization calculations which must not be destroyed between calls of subroutine RFFTF1 or RFFTB1.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::rfftf1`
- Original SLATEC routine: `RFFTF1`
- Native symbol: `rfftf1_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [RFFTF1](https://www.netlib.org/slatec/fishfft/rfftf1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
