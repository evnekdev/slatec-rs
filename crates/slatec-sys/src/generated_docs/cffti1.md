# Purpose

Subroutine CFFTI1 initializes the work arrays WA and IFAC which are used in both CFFTF1 and CFFTB1. The prime factorization of N and a tabulation of the trigonometric functions are computed and stored in

# Description

This canonical unsafe binding exposes original SLATEC routine `CFFTI1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [CFFTI1](https://www.netlib.org/slatec/fishfft/cffti1.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. the length of the sequence to be transformed the length of the sequence to be transformed not applicable or not stated by selected source not a workspace argument

## 2. `WA`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). a real work array which must be dimensioned at least 2*N. must not be changed between calls of CFFTF1 or CFFTB1. a real work array which must be dimensioned at least 2*N. must not be changed between calls of CFFTF1 or CFFTB1. not applicable or not stated by selected source

## 3. `IFAC`

output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). and WA, respectively. Input Parameter an integer work array which must be dimensioned at least 15. The same work arrays can be used for both CFFTF1 and CFFTB1 as long as N remains unchanged.  Different WA and IFAC arrays are required for different values of N.  The contents of must not be changed between calls of CFFTF1 or CFFTB1. and WA, respectively. Input Parameter an integer work array which must be dimensioned at least 15. The same work arrays can be used for both CFFTF1 and CFFTB1 as long as N remains unchanged.  Different WA and IFAC arrays are required for different values of N.  The contents of must not be changed between calls of CFFTF1 or CFFTB1. not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `WA`: a real work array which must be dimensioned at least 2*N. must not be changed between calls of CFFTF1 or CFFTB1.
- `IFAC`: and WA, respectively. Input Parameter an integer work array which must be dimensioned at least 15. The same work arrays can be used for both CFFTF1 and CFFTB1 as long as N remains unchanged.  Different WA and IFAC arrays are required for different values of N.  The contents of must not be changed between calls of CFFTF1 or CFFTB1.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::cffti1`
- Original SLATEC routine: `CFFTI1`
- Native symbol: `cffti1_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [CFFTI1](https://www.netlib.org/slatec/fishfft/cffti1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
