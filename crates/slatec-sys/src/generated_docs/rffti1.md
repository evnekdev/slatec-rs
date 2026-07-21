# Purpose

Subroutine RFFTI1 initializes the work arrays WA and IFAC which are used in both RFFTF1 and RFFTB1. The prime factorization of N and a tabulation of the trigonometric functions are computed and stored in IFAC and WA, respectively. Input Argument

# Description

This canonical unsafe binding exposes original SLATEC routine `RFFTI1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RFFTI1](https://www.netlib.org/slatec/fishfft/rffti1.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the length of the sequence to be transformed. Output Arguments.

## `WA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a real work array which must be dimensioned at least N.

## `IFAC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

an integer work array which must be dimensioned at least 15. The same work arrays can be used for both RFFTF1 and RFFTB1 as long as N remains unchanged. Different WA and IFAC arrays are required for different values of N. The contents of WA and IFAC must not be changed between calls of RFFTF1 or RFFTB1.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `WA`: a real work array which must be dimensioned at least N.
- `IFAC`: an integer work array which must be dimensioned at least 15. The same work arrays can be used for both RFFTF1 and RFFTB1 as long as N remains unchanged. Different WA and IFAC arrays are required for different values of N. The contents of WA and IFAC must not be changed between calls of RFFTF1 or RFFTB1.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::rffti1`
- Original SLATEC routine: `RFFTI1`
- Native symbol: `rffti1_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [RFFTI1](https://www.netlib.org/slatec/fishfft/rffti1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
