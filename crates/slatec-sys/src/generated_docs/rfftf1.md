# Purpose

Subroutine RFFTF1 computes the Fourier coefficients of a real periodic sequence (Fourier analysis). The transform is defined below at output parameter C. The arrays WA and IFAC which are used by subroutine RFFTB1 must be initialized by calling subroutine RFFTI1. Input Arguments

# Description

This canonical unsafe binding exposes original SLATEC routine `RFFTF1`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RFFTF1](https://www.netlib.org/slatec/fishfft/rfftf1.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the length of the array R to be transformed. The method is most efficient when N is a product of small primes. N may change so long as different work arrays are provided.

## `C`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a real array of length N which contains the sequence to be transformed. the sum from I=1 to I=N of R(I) If N is even set L = N/2; if N is odd set L = (N+1)/2 then for K = 2,. ,L the sum from I = 1 to I = N of C(I)*COS((K-1)*(I-1)*2*PI/N) -C(I)*SIN((K-1)*(I-1)*2*PI/N) (-1)**(I-1)*C(I) Notes: This transform is unnormalized since a call of RFFTF1 followed by a call of RFFTB1 will multiply the input sequence by N.

## `CH`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a real work array of length at least N.

## `WA`

**Direction:** `input`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a real work array which must be dimensioned at least N. initialization calculations which must not be destroyed between calls of subroutine RFFTF1 or RFFTB1.

## `IFAC`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

an integer work array which must be dimensioned at least 15. The WA and IFAC arrays must be initialized by calling subroutine RFFTI1, and different WA and IFAC arrays must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WA and IFAC arrays can be used by RFFTF1 and RFFTB1. Output Argument initialization calculations which must not be destroyed between calls of subroutine RFFTF1 or.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `C`: not a workspace argument
- `CH`: a real work array of length at least N.
- `WA`: a real work array which must be dimensioned at least N. initialization calculations which must not be destroyed between calls of subroutine RFFTF1 or RFFTB1.
- `IFAC`: an integer work array which must be dimensioned at least 15. The WA and IFAC arrays must be initialized by calling subroutine RFFTI1, and different WA and IFAC arrays must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WA and IFAC arrays can be used by RFFTF1 and RFFTB1. Output Argument initialization calculations which must not be destroyed between calls of subroutine RFFTF1 or.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::rfftf1`
- Original SLATEC routine: `RFFTF1`
- Native symbol: `rfftf1_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`
- Exact Netlib source file: [RFFTF1](https://www.netlib.org/slatec/fishfft/rfftf1.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
