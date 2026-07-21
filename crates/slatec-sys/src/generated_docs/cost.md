# Purpose

Subroutine COST computes the discrete Fourier cosine transform of an even sequence X(I). The transform is defined below at output parameter X. COST is the unnormalized inverse of itself since a call of COST followed by another call of COST will multiply the input sequence X by 2*(N-1). The transform is defined below at output parameter X. The array WSAVE which is used by subroutine COST must be initialized by calling subroutine COSTI(N,WSAVE).

# Description

This canonical unsafe binding exposes original SLATEC routine `COST`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [COST](https://www.netlib.org/slatec/fishfft/cost.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

the length of the sequence X. N must be greater than 1. The method is most efficient when N-1 is a product of small primes.

## `X`

**Direction:** `input-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

an array which contains the sequence to be transformed For I=1,. ,N X(1)+(-1)**(I-1)*X(N) + the sum from K=2 to K=N-1 2*X(K)*COS((K-1)*(I-1)*PI/(N-1)) A call of COST followed by another call of COST will multiply the sequence X by 2*(N-1). Hence COST is the unnormalized inverse of itself.

## `WSAVE`

**Direction:** `workspace-output`. **Fortran type:** `REAL`. **Rust ABI type:** `*mut f32`. **Shape:** rank 1; dimensions (*).

a work array which must be dimensioned at least 3*N+15 in the program that calls COST. The WSAVE array must be initialized by calling subroutine COSTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. contains initialization calculations which must not be destroyed between calls of COST.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Workspace and array requirements

- `X`: not a workspace argument
- `WSAVE`: a work array which must be dimensioned at least 3*N+15 in the program that calls COST. The WSAVE array must be initialized by calling subroutine COSTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. contains initialization calculations which must not be destroyed between calls of COST.

# ABI notes

- Canonical Rust path: `slatec_sys::fftpack::cost`
- Original SLATEC routine: `COST`
- Native symbol: `cost_`
- ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`
- Exact Netlib source file: [COST](https://www.netlib.org/slatec/fishfft/cost.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
