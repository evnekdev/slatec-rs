# Purpose

Subroutine CFFTF1 computes the forward complex discrete Fourier transform (the Fourier analysis). Equivalently, CFFTF1 computes the Fourier coefficients of a complex periodic sequence. The transform is defined below at output parameter C. The transform is not normalized. To obtain a normalized transform the output must be divided by N. Otherwise a call of CFFTF1 followed by a call of CFFTB1 will multiply the sequence by N.

# Description

This canonical unsafe binding exposes original SLATEC routine `CFFTF1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CFFTF1](https://www.netlib.org/slatec/fishfft/cfftf1.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. the length of the complex sequence C.  The method is more efficient when N is the product of small primes. contains the sequence the length of the complex sequence C.  The method is more efficient when N is the product of small primes. contains the sequence not applicable or not stated by selected source not a workspace argument

## 2. `C`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). contains the sequence 1,...,N the sum from K=1,...,N of I*(J-1)*(K-1)*2*PI/N) where I=SQRT(-1) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 3. `CH`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). a real work array of length at least 2*N a real work array of length at least 2*N not applicable or not stated by selected source

## 4. `WA`

input `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC). a real work array which must be dimensioned at least 2*N. must be initialized by calling subroutine CFFTI1 (N, WA, IFAC), and different WA and IFAC arrays must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first.  The same WA and IFAC arrays can be used by CFFTF1 and CFFTB1. contain initialization calculations which must not be destroyed between calls of subroutine CFFTF1 or CFFTB1 are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC). a real work array which must be dimensioned at least 2*N. must be initialized by calling subroutine CFFTI1 (N, WA, IFAC), and different WA and IFAC arrays must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first.  The same WA and IFAC arrays can be used by CFFTF1 and CFFTB1. contain initialization calculations which must not be destroyed between calls of subroutine CFFTF1 or CFFTB1 not applicable or not stated by selected source

## 5. `IFAC`

input `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC). an integer work array which must be dimensioned at least 15. must be initialized by calling subroutine CFFTI1 (N, WA, IFAC), and different WA and IFAC arrays must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first.  The same WA and IFAC arrays can be used by CFFTF1 and CFFTB1. contain initialization calculations which must not be destroyed between calls of subroutine CFFTF1 or CFFTB1 are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC). an integer work array which must be dimensioned at least 15. must be initialized by calling subroutine CFFTI1 (N, WA, IFAC), and different WA and IFAC arrays must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first.  The same WA and IFAC arrays can be used by CFFTF1 and CFFTB1. contain initialization calculations which must not be destroyed between calls of subroutine CFFTF1 or CFFTB1 not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `C`: not a workspace argument
- `CH`: a real work array of length at least 2*N
- `WA`: are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC). a real work array which must be dimensioned at least 2*N. must be initialized by calling subroutine CFFTI1 (N, WA, IFAC), and different WA and IFAC arrays must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first.  The same WA and IFAC arrays can be used by CFFTF1 and CFFTB1. contain initialization calculations which must not be destroyed between calls of subroutine CFFTF1 or CFFTB1
- `IFAC`: are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC). an integer work array which must be dimensioned at least 15. must be initialized by calling subroutine CFFTI1 (N, WA, IFAC), and different WA and IFAC arrays must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first.  The same WA and IFAC arrays can be used by CFFTF1 and CFFTB1. contain initialization calculations which must not be destroyed between calls of subroutine CFFTF1 or CFFTB1

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::cfftf1`
- Original SLATEC routine: `CFFTF1`
- Native symbol: `cfftf1_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [CFFTF1](https://www.netlib.org/slatec/fishfft/cfftf1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
