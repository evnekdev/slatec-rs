# Purpose

Subroutine COSTI initializes the array WSAVE which is used in subroutine COST. The prime factorization of N together with a tabulation of the trigonometric functions are computed and stored in WSAVE. Input Parameter

# Description

This canonical unsafe binding exposes original SLATEC routine `COSTI`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [COSTI](https://www.netlib.org/slatec/fishfft/costi.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the length of the sequence to be transformed. The method is most efficient when N-1 is a product of small primes. Output Parameter.

## `WSAVE`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a work array which must be dimensioned at least 3*N+15. Different WSAVE arrays are required for different values of N. The contents of WSAVE must not be changed between calls of COST.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `WSAVE`: a work array which must be dimensioned at least 3*N+15. Different WSAVE arrays are required for different values of N. The contents of WSAVE must not be changed between calls of COST.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::costi`
- Original SLATEC routine: `COSTI`
- Native symbol: `costi_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1)`
- Exact Netlib source file: [COSTI](https://www.netlib.org/slatec/fishfft/costi.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
