# Purpose

Subroutine RFFTI1 initializes the work arrays WA and IFAC which are used in both RFFTF1 and RFFTB1. The prime factorization of N and a tabulation of the trigonometric functions are computed and stored in

# Description

This canonical unsafe binding exposes original SLATEC routine `RFFTI1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RFFTI1](https://www.netlib.org/slatec/fishfft/rffti1.f).

# Arguments

## 1. `N`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. the length of the sequence to be transformed. Output Arguments are required must not be changed between calls of RFFTF1 or RFFTB1. the length of the sequence to be transformed. Output Arguments are required must not be changed between calls of RFFTF1 or RFFTB1. not applicable or not stated by selected source not a workspace argument

## 2. `WA`

output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (*). a real work array which must be dimensioned at least N. are required must not be changed between calls of RFFTF1 or RFFTB1. a real work array which must be dimensioned at least N. are required must not be changed between calls of RFFTF1 or RFFTB1. not applicable or not stated by selected source

## 3. `IFAC`

input-output `array` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and rank 1; dimensions (*). and WA, respectively. Input Argument an integer work array which must be dimensioned at least 15. The same work arrays can be used for both RFFTF1 and RFFTB1 as long are required must not be changed between calls of RFFTF1 or RFFTB1. and WA, respectively. Input Argument an integer work array which must be dimensioned at least 15. The same work arrays can be used for both RFFTF1 and RFFTB1 as long are required must not be changed between calls of RFFTF1 or RFFTB1. not applicable or not stated by selected source

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `N`: not a workspace argument
- `WA`: a real work array which must be dimensioned at least N. are required must not be changed between calls of RFFTF1 or RFFTB1.
- `IFAC`: and WA, respectively. Input Argument an integer work array which must be dimensioned at least 15. The same work arrays can be used for both RFFTF1 and RFFTB1 as long are required must not be changed between calls of RFFTF1 or RFFTB1.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::rffti1`
- Original SLATEC routine: `RFFTI1`
- Native symbol: `rffti1_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [RFFTI1](https://www.netlib.org/slatec/fishfft/rffti1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
